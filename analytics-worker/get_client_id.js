<!-- GET CLIENT_ID -->
function getClientId(measurementId) {
    const script = document.createElement("script");
    script.type = "text/javascript";
    document.head.appendChild(script);
    script.src = `https://www.googletagmanager.com/gtag/js?id=G-M29Y3ELKER`;

    function gtag() {
        dataLayer.push(arguments);
    }

    gtag('js', new Date());

    gtag('config', measurementId);
    gtag('get', measurementId, 'client_id', client_id => {
        console.log(client_id);
    });
}

const MEASUREMENT_ID = 'G-M29Y3ELKER';
getClientId(MEASUREMENT_ID);
