# my-domains

It performs a series of whois request for the domains specified in 
settings.toml and tries to extract the expiration date.

The settings.toml should have the following format:
```
address = "0.0.0.0:7878"
domains = [
  "my-domain.net",
  "my-other-domain.com",
  "my-european-domains.eu",
]
```

