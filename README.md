# What is this?

I'm trying to replicate an issue I'm having with [Tower
Sessions](https://github.com/maxcountryman/tower-sessions), where the session seems to be missing
data upon initial return from Stripe, but if you manually enter the URL, it works.

# How to run

In order to demonstrate the issue, the "stripe" service must be run on a different origin than the
"princess" service. The following will set that up:

`cargo run --bin=stripe & cargo run --bin=princess -- http://$(hostname):4001/`

Now visit http://localhost:4000/ and click the
buttons. At the end, you'll end up at a page at http://localhost:4000/success and see the test data
inserted in the initial get of `/`, or a message indicating failure. To see it succeed, run:

`cargo run --bin=stripe & cargo run --bin=princess`

and follow the buttons starting from http://localhost:4000/ again to the end.

# Why is it called "princess"?

Because it's the baby version of [Queenie](https://git.kittenclause.com/nebkor/queen), a small
service using the same crates that sends users to Stripe and then receives a redirect from there
once a user has successfully paid. I just wanted to get a minimum test case.
