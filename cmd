 $ curl -F "f=@filename.ext" gbin.me

* This paste will expire after 69 seconds. Max Value: 18446744073709551615
--------------------------------------------------------------------------
$ curl -F "f=@filename.ext" -F "expire=69" gbin.me

* This paste will expire after 3 reads.
--------------------------------------
$ curl -F "f=@filename.ext" -F "read=3" gbin.me

* If you don't want the resulting URL to be easy to guess.
  You can chose a URL length between 8 to 255.
---------------------------------------------------------
$ curl -F "f=@filename.ext" -F "deepurl=3" gbin.me

* You can delete a paste only if a secret value was set.
  If you haven't provided any secret while creating the
  paste, it will be automatically deleted after 69 days.
--------------------------------------------------------
$ curl -F "f=@filename.ext" -F "secret=password" gbin.me

* Now in order to delete it you have to mention the secret
  and use the DELETE method on the paste URL.
--------------------------------------------------------
$ curl -XDELETE -F "secret=password" gbin.me/pasteid

EXAMPLE
* This will create a URL of length 12. It will expire after
  69 seconds and you can only view it 1 time.
-----------------------------------------------------------
$ curl -F "f=@filename.ext" -F "deepurl=12" -F "expire=69" -F "read=1" gbin.me