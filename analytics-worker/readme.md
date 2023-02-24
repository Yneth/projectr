# Description

Sample application to send Google Analytics using [measurement protocol](https://developers.google.com/analytics/devguides/collection/protocol/ga4).

# Result
Screenshot from `Reports > Realtime`.

![exchange_rate_realtime_events.png](exchange_rate_realtime_events.png)

Was not able to see events from `Admin > Events` tab, need to wait for 24h.

# Key points
* Use GA4 protocol as universal metrics are being deprecated;
* To generate client_id use gtag script;
* To debug correctness of event request, use `/debug` at the beginning of the uri;
* To validate event sending, use `Reports > Realtime` tab;
