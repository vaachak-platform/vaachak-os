Validation notes

1. Open Wi-Fi Transfer and hard-refresh browser:
   http://x4.local/?v=transfer-reader-fix

2. To verify /FCACHE/15D1296A manually:
   curl "http://x4.local/v2/stat?p=%2FFCACHE%2F15D1296A%2FMETA.TXT"

3. Expected JSON when file exists:
   {"exists":true,"size":141}

4. Reader expected outcomes:
   Prep Pg ...
   or
   Read cache:15D1296A err:INDEX Pg ...
   Read cache:15D1296A err:FONT Pg ...
   Read cache:15D1296A err:PAGE Pg ...
   Read cache:15D1296A err:MISSING Pg ...
