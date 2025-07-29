# ⚔️ Sword Mache: Game of Infamy!

**Sword Mache** is a terminal-based arcade game built with [Rust] and designed for live-coding presentations. It features real-time movement, combat with monsters, and retro-style ASCII graphics. Play it to learn Rust, test your reflexes, or just enjoy a brutal ASCII brawl.

---

## 🚀 Getting Started

### 1. 📦 Install Rust

Make sure you have Rust installed. If not, [install Rust here].

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 🐧 Linux Users

Install required [Linux dependencies] for [rusty_audio], such as `libasound2-dev`:

```sh
sudo apt-get install libasound2-dev
```

### 3. ⬇️ Clone the Repository

```sh
git clone https://github.com/abhijett55/sword_mache.git
cd sword_mache
```

### 4. 🛠️ Build & Play the Game

```sh
cargo run --release
```

---

## 🎮 Gameplay Instructions

- You are represented by the symbol `☥` and your **rusty sword** points in the direction you're moving.
- **Move** with arrow keys or `WASD` keys.
- Your sword will automatically point in the direction you move — no fancy swordplay here.
- **Kill monsters** by touching them with your sword.
- **Avoid getting touched** by monsters — they'll instantly kill you!
- Survive as long as you can!

---

## 🔧 Development Info

This project is modular and written entirely in Rust. Key components include:

- `coord`: Grid-based coordinate system
- `floor`: Randomized map generation
- `player`: Movement and state
- `monster`: AI for chasing the player
- `render`: Draws the terminal UI
- `timer`: Game loop and frame timing
- `world`: Combines all the modules into a game world

---

## 📚 References

- [Rust Language]
- [rusty_audio]: A minimalist audio engine for terminal games
- [Linux Dependencies for rusty_audio]

---

## 🧠 License / Contribution

Feel free to fork, modify, and contribute to this game! Ideal for Rust learners or arcade game lovers.

---

[Install Rust]: https://www.rust-lang.org/tools/install
[Rust]: https://www.rust-lang.org/
[rusty_audio]: https://github.com/CleanCut/rusty_audio
[Linux dependencies]: https://github.com/CleanCut/rusty_audio#dependencies-on-linux
