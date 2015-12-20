# cs-eco-dash

A CS:GO economy dashboard.

# Usage

Once this is useful, there will be binary releases.
This is not currently useful, so there are no binary releases.

```shell
git clone https://github.com/mathphreak/cs-eco-dash.git
cp config/gsi.cfg $CSGO_DIR/cfg/gamestate_integration_csecodash.cfg
cargo build --release
```

Then run `target/release/cs-eco-dash`.

Point a Web browser to `localhost:3000`, either on a second monitor or in
the Steam overlay.

Start a game and watch things happen!
