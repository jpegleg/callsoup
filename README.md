![cdlogo](https://carefuldata.com/images/cdlogo.png)

# callsoup

Callsoup is a variation of morpho-web (https://github.com/jpegleg/morpho-web) that
adds crates blake3, redis, reqwest, and actix-http, and removes the flying-squirrel-tactix health check.
With that healthcheck removed, callsoup has dropped curl libraries. Instead of using curl,
callsoup uses reqwest for callbacks. Callsoup also supports processing of request bodies via
actix-http and bytes crates. Another difference is binding to port 3333 instead of 443.

## Configuration

There are a few required files:

```
- cert.pem - the server identity certificate as a PKCS8
- key.pem - the key used to create the server identity certificate
- static/index.html - the default GET response, any other web pages within static/ are optional
```


## About the template

The callsoup template includes two modules which represent the two function demos:

stor.rs which fetches values from redis, with the redis key provided in a POST body

fetch.rs which sends a callback to a URL provided in a POST body

The example functionality with fetch inserts a BLAKE3 hash of the callback GET response body
into redis, with the URL being the key. And then the stor functionality can retrieve the
hash by requesting the URL. This can be utlized for monitoring/observability purposes,
but is really just an example that can be further modified to build out more advanced
callback type functionality. 

The POST body data in these two example functionalities is straight text in, JSON out.
`$CALLBACK_TARGET` would be the place to send your callback to, such as `https://somemetrics.someplace.somewhere/api/v1/metrics` or whatever the
DNS name is for the system you need to send the callback or healthcheck to, etc etc. Then `$CALLSOUPURL` would be the endpoint you have
for your callsoup service, such as `https://myplace.local:3333` or however you service it up. It could also be used on a loopback device, or
different port, just change the bind stanza if such a thing is needed. 

These are cURL examples, but potentialy instead of curl, you might more likely bake in HTTP requests to the callsoup service for
your other applications. 

```
curl -X POST -H "Content-Type: application/json" --data "$CALLBACK_TARGET" "$CALLSOUPURL"/api/fetch
curl -X POST -H "Content-Type: application/json" --data "$CALLBACK_TARGET" "$CALLSOUPURL"/api/storage
```

The fetch output includes a UUID which can be used to trace the transaction on the server side.
The JSON response structure in the example template is simple:

fetch success: `{ "Ok(())": "809b1d8e-75a5-4e52-a1e3-a02d68de0b37" }`

fetch error: `{ "ERROR": "e6fa4097-9aa6-42fb-91f2-41acaea125d3" }`

stor success (the hash will be different depending on the HTTP response of course): `{ "data": "984862d357e43075d4b5ad07dcac7729625f9c4b7c9c35f3ec07999530cb4585" }`

stor error: `{ "data": "Not found." }`

...

I expect I'll include another example module that parses JSON body input, and perhaps add JSON 
body input support to the stor and fetch modules.

The redis is designed to be on the loopback with callsoup, so using the same Pod in Kubernetes
or otherwise calling redis locally. This can be switched out to remote redis if needed. 
I like keeping caches in the same Pod so there isn't network overhead/risks for redis.

Callsoup does not have authentication in the template, add whichever auth mechanisms you like.

