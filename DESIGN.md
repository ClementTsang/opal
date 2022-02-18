# Design

Some notes on design decisions.

## Hosting a database as part of the static page

So this is a weird design choice. Why not just upload the database to some service and use API calls to query? Well, the
basic answer is that I didn't want to spend money on a service, both because I'm a cheapskate, and because I was worried
I would forget about it. I don't want to spend money forever for a service I forget about, nor do I want the service
to go down mysteriously because I accidentally forget to pay for it.

However, I didn't want to just send the entire database to a user. That's the easy way out, but that's... kinda a last
resort in my opinion. Having a user have to download the _entire_ English database on every page load isn't the greatest.

And lastly, it just seems kinda cool to try it like this.
