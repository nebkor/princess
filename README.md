# What is this?

I'm trying to replicate an issue I'm having with [Tower
Sessions](https://github.com/maxcountryman/tower-sessions), where the session seems to be missing
data upon initial return from Stripe, but if you manually enter the URL, it works.

# How to run

First, run `cargo run --bin=stripe &`; this will start the "Stripe" service listening on
localhost:4001. Then, run `cargo run --bin=princess` and visit http://localhost:4000/ and click the
buttons. At the end, you'll end up at a page at http://localhost:4000/success and see the test data
inserted in the initial get of `/`.

# Why is it called "princess"?

Because it's the baby version of [Queenie](https://git.kittenclause.com/nebkor/queen), a small
service using the same crates that sends users to Stripe and then receives a redirect from there
once a user has successfully paid. I just wanted to get a minimum test case.
