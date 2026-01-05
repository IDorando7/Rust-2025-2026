# Trap The Mouse (Rust) — Server + Client (egui) + PvP + vs_bot

Proiect: implementarea jocului “Trap The Mouse” cu:
- **server** (Axum WebSocket) care găzduiește camere și validează mutările
- **client** (egui/eframe) care se conectează la server și joacă
- mod **PvP**: Trapper vs Mouse
- mod **vs_bot**: Trapper vs “mouse controlat de server” (bot)

> Notă: Clientul se conectează mereu la server. În `vs_bot`, serverul calculează mutările mouse-ului.

---

## Cerințe / Dependențe
- Rust toolchain (stable)
- `cargo` (workspace)
- Pentru GUI în WSL: WSLg sau un setup care permite ferestre GUI.

---

## Cum rulezi serverul
Din root-ul proiectului:

```bash
cargo run -p server
```

Serverul pornește la:
- WebSocket: `ws://127.0.0.1:3000/ws`
- Healthcheck: `http://127.0.0.1:3000/health`

Alternativ, folosește scriptul:
```bash
./scripts/run_server.sh
```

---

## Cum rulezi clientul
Din root:

```bash
cargo run -p client
```

Alternativ:
```bash
./scripts/run_client.sh
```

Clientul are input URL (implicit `ws://127.0.0.1:3000/ws`).

---

## Cum testezi vs_bot
1) Pornește serverul
2) Pornește clientul
3) Connect
4) În Lobby: creează o cameră cu `vs_bot = true` (checkbox)
5) Start game:
    - tu ești **Trapper**
    - după ce pui un block, serverul mută automat mouse-ul (bot)

---

## Cum testezi PvP (două instanțe client)
1) Pornește serverul
2) Pornește două instanțe de client (două ferestre):
    - manual: rulezi `cargo run -p client` de două ori
    - sau script:
      ```bash
      ./scripts/run_two_clients.sh
      ```
3) În primul client: Create room `vs_bot=false`
4) În al doilea client: Join la room-ul respectiv
5) Jocul pornește automat:
    - un client devine Trapper
    - celălalt devine Mouse

---

## Protocol (ClientMsg / ServerMsg) — pe scurt
Comunicarea este JSON peste WebSocket.

### Client -> Server (`ClientMsg`)
- `ListRooms`
- `CreateRoom { name, vs_bot }`
- `JoinRoom { room_id }`
- `LeaveRoom`
- `PlayerAction { action }`

### Server -> Client (`ServerMsg`)
- `RoomList { rooms }` — lista camerelor pentru lobby
- `LobbyState { room_id, players, vs_bot }` — update când intră/iese cineva din cameră
- `GameStart { state, your_role }` — începe jocul + rolul clientului
- `GameUpdate { state }` — update de stare după o mutare validă
- `Error { message }`

---

## Reguli joc (validare pe server)
Serverul validează mutările folosind `shared::rules::apply_action`.

- Trapper: `PlaceBlock { at }`
    - doar dacă e rândul lui
    - `at` e pe tablă, nu e blocat, nu e celula mouse-ului

- Mouse: `MoveMouse { to }`
    - doar dacă e rândul lui
    - `to` e pe tablă, nu e blocat, și e vecin cu mouse-ul curent

Endgame:
- Mouse câștigă dacă ajunge pe margine (`is_border`)
- Trapper câștigă dacă mouse nu mai are mutări legale

---

## “Angel Problem” (Wikipedia) — legătura cu proiectul
Jocul este inspirat conceptual de “Angel Problem” (joc pe grilă infinită cu obstacole).

În acest proiect:
- **nu** rezolvăm problema teoretică la nivel research
- în modul `vs_bot`, bot-ul pentru mouse folosește un algoritm simplu: **BFS către margine** (vezi `shared::ai`)
- serverul “dictează” mutările mouse-ului în `vs_bot`, păstrând logica centralizată și consistentă.

Resursă: Wikipedia — Angel problem
