A [Dark Souls random name generator](https://helo.randomnumberpicker.co.uk).

Written in Rust using Iron; responds with JSON to get or post requests.

```
// get
https://helo.randomnumberpicker.co.uk
=> {"name":"Parasitic Brian, Captain of the Abyss","success":true,"error_message":""}

// get
https://helo.randomnumberpicker.co.uk/Sebastian
=> {"name":"Iron Sebastian","success":true,"error_message":""}

// post
curl -X POST -d '{"name":"Sandra"}' https://helo.randomnumberpicker.co.uk
=> {"name":"Sandra, Oracle of the Sunless Realms","success":true,"error_message":""}
```

Write-up in `/notes`