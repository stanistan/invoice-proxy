# invoice-proxy

This project is a HTTP proxy for [airtable](http://airtable.com), specifically
to stitch together a DAG of queries/resources into one large endpoint.

I'm using this specfically to build invoice templates powered by Airtable.

## Usage

You need two environment variables to use this:

1. `AIRTABLE_KEY` - your airtable API key
2. `AIRTABLE_APP` - the base/app of your specific airtable

The default port that this runs on is `3000`, and can be overriden setting the `PORT`
environment variable.

## Schema

The Resources are defined in `src/schema.rs`.

## Endpoints

- `GET /invoice/{id}` - gets an invoice
- `GET cache/stats` - provides the cache hits/misses for the local state of the server
- `GET cache/clear` - will clear the response cache
