# cs-eco-dash

A CS:GO economy dashboard.

# Usage

Once this is useful, there will be binary releases.
This is not currently useful, so there are no binary releases.

```shell
git clone https://github.com/mathphreak/cs-eco-dash.git
cargo build --release
```

Then run `target/release/cs-eco-dash`. This will (ideally) launch a Web browser
pointed at `localhost:3000`.

Press the "Install/Update" button if it's visible.

Launch CS:GO. Keep that Web browser open in a second monitor, or close it and open `localhost:3000` in the Steam overlay.

Start a game and watch things happen!

# How It Works

Uses Valve's [Game State Integration][] to get data.

[Game State Integration]: https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration
