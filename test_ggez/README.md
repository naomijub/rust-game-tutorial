# Test_ggez

A testing framework for GGEZ Games inpired by [`xray`](https://gitlab.com/tonyfinn/xray)

See [`examples`](https://github.com/naomijub/rust-game-tutorial/tree/main/game/examples) folder in [`game`](https://github.com/naomijub/rust-game-tutorial/tree/main/game) to understand usage.

* Tests are run in the example folder because `main` threat is necessary for context.
* First time you run a new test, a PNG will be added with the name `expected.png` to `test_resources/<test name>/` with the captured screenshot.
* If the expected screenshot is equal to the current screenshot, the test is a success.
* If the expected screenshot is **NOT** equal to the current screenshot, two new files will be added to `test_resources/<test name>/`:
    1. `actual.png` with the current screenshot.
    2. `diff.png` with the difference between actual and expected.