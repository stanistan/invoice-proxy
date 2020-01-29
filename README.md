# An invoice-proxy

This is a real simple proxy to airtable that formats an invoice into a
_one_ JSON response.

## There are two versions of this...

There are two branches,

- `php`,
- `rustlang`

Originally, I built this in PHP to just get this to work, and needed
to have a separate "server" setup to execute the PHP scripts.

I decided to experiment with having a single rust executable for this.

The differences in the output of the two implementations right now is
that for the rust version, the keys are re-named, and it uses a shared
in memory cache for all requests, while the PHP version maintains the same
response keys as airtable provides, and its cache is on disk, since
PHP doesn't share memory between requests.
