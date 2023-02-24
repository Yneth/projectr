import datetime
import logging
import os

import requests
from requests.adapters import Retry, HTTPAdapter

TIMEOUT_SEC = 10
RETRY = Retry(total=5,
              backoff_factor=0.1,
              status_forcelist=[500, 502, 503, 504])


def get_exchange_rate(session, date):
    """
    queries Privat24 API currency exchange pairs.
    filters only UAH USD pairs and returns object as is.
    """

    logging.info("getting exchange rate for date %s", date)

    date_str = date.strftime("%d.%m.%Y")
    uri = f"https://api.privatbank.ua/p24api/exchange_rates?json&date={date_str}"

    response = session.get(uri, timeout=10)
    if response.status_code > 299:
        raise Exception(f"privat24 API: Received {response.status_code} {response.json()}")

    logging.info("getting exchange rate for date %s - success", date)

    json = response.json()
    for exchange_pair in json["exchangeRate"]:
        if exchange_pair["baseCurrency"] == "UAH" and exchange_pair["currency"] == "USD":
            return exchange_pair

    raise Exception("No currency pair found")


def send_analytics(session, measurement_id, api_secret, client_id, is_debug, exchange_pair):
    debug_api = ""
    if is_debug:
        debug_api = "/debug"

    uri = f"https://www.google-analytics.com{debug_api}/mp/collect?measurement_id={measurement_id}&api_secret={api_secret}"
    body = {
        "client_id": client_id,
        "events": [{
            "name": "exchange_rate",
            "params": exchange_pair
        }]
    }
    logging.debug("analytics API: request body=%s", body)
    response = session.post(url=uri, json=body, timeout=10)
    logging.debug(f"analytics API: status=%s response=%s", response.status_code, response.text)

    if response.status_code > 299:
        raise Exception(f"API: Received {response.status_code} {response.json()}")


def months(year_from, year_to):
    for year in range(year_from, year_to + 1):
        for month in range(1, 13):
            yield datetime.date.today().replace(year=year, month=month, day=1)


def main():
    logging.basicConfig(level=logging.DEBUG)
    try:
        with requests.Session() as session:
            measurement_id = os.getenv("GA4_MEASUREMENT_ID")
            api_secret = os.getenv("GA4_API_SECRET")
            client_id = os.getenv("GA4_CLIENT_ID")
            is_debug = os.getenv("DEBUG")

            session.mount(prefix='https://', adapter=HTTPAdapter(max_retries=RETRY))
            for first_day_of_month in months(2015, 2022):
                exchange_pair = get_exchange_rate(session, first_day_of_month)
                send_analytics(session, measurement_id, api_secret, client_id, is_debug, exchange_pair)
    except Exception as e:
        logging.error("failed to transfer exchange rates, reason", e)


if __name__ == "__main__":
    main()
