4.18.2 / 2022-10-08
===================

  * Fix regression routing a large stack in a single route
  * Fix `req.resume()` called after `res.pipe()`
  * deps: body-parser@1.20.1
    - deps: qs@6.11.0
  * deps: qs@6.11.0

4.18.1 / 2022-04-29
===================

  * Fix hanging on large stack of sync routes

4.18.0 / 2022-04-25
===================

  * Add `res.download` support for object with all optional properties
  * Change `res.sendFile` to use `send@0.18.0`
  * deps: body-parser@1.20.0
  * Remove deprecated leading colon in `name` for `app.param(name, fn)`
