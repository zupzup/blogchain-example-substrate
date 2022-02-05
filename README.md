# Blogchain


An example for a Substrate-Node-Template backed custom blockchain


Compile with:

```bash
cargo build --release
```

Start with:

```bash
./target/release/node-template --dev
```

Then you can either use [https://github.com/substrate-developer-hub/substrate-front-end-template](https://github.com/substrate-developer-hub/substrate-front-end-template) to start up a frontend to connect to the blockchain, or you can use [https://polkadot.js.org/apps/#/explorer](https://polkadot.js.org/apps/#/explorer) in `Development` mode using `Local Node`

## Functionality

There is a custom pallet called `blogchain` implemented, which features a simplistic blogging service.

In terms of extrinsics (functions you can use to interact with the blockchain), you can do three things:

* Create a new blog post (blogchain -> `create_blog_post`) - enter an arbitrary string between 64 and 4096 bytes as the content of your post
    * Costs: 10000
* Comment on an existing blog post (blogchain -> `create_blog_post_comment`) - enter a blog post id and an arbitrary string between 64 and 1024 bytes - this comment will be saved in a list on the blog post
    * Costs: 5000
* Tip the blog post author (blogchain -> `tip_blog_post`) - enter a blog post id and an amount to send
    * Costs: 500 (+amount you wish to send, obviously)
