# 9box  
**Design Document for Rust Version of the Succession-Planning Tool**

---

## 1. Project Overview

This tool provides a desktop-grade, cross-platform interface for 9-box succession planning, now built entirely in Rust. Users can:

- Import personnel data from CSV (or future API endpoints).  
- See draggable “employee cards” and a 9-box grid for performance × potential assessments.  
- Assign and remove skill tags via drag-and-drop.  
- Edit and persist notes on each card.  

By targeting Rust and a native GUI toolkit, this version ships to macOS, Windows, and Linux from one codebase (no Python, no Electron, no Xcode required).

---

## 2. Features & Requirements

1. **Data Import & Export**  
   - Read employee CSVs (Name, Position, PR score, Skills, Notes, Box) via `csv` + `serde`.  
   - Write back to CSV, preserving all fields.  
   - Future-proof for REST/GraphQL API hooks.

2. **UI Layout**  
   - **Left panel** with two tabs: “Employees” and “Skills.”  
   - **9-box grid** in the main area, cells labeled and resizable.  
   - **Draggable cards** that stack with scrollbars on overflow.  
   - **Collapsible** left panel when empty; toggleable via button.

3. **Card Details**  
   - Shows Name, Position, PR score, skill-tag list, and a notes icon.  
   - Click a card to expand inline notes (editable text area).  
   - Skill tags removable with an “×” button.

4. **Drag-and-Drop & Dynamic Behavior**  
   - Native drag events for cards ↔ grid and skills ↔ cards.  
   - Animated feedback on drop.  
   - “Refresh” button to reset to an initial import state.

---

## 3. Updated Features

Based on latest UI sketches and feedback:

- **Multi-skill drags**: drop several skills at once without overwriting existing tags.  
- **Persistent notes**: auto-save per user ID (backed by a simple local JSON or SQLite store).  
- **Responsive grid**: auto-layout on window resize; icons update live.  
- **Dark/light themes**: toggleable from the application menu.

---

## 4. Implementation Plan

### 4.1 Core Library (`box_planner-core`)  
- **Data models** in `src/model.rs` (derive `Serialize`/`Deserialize`).  
- **CSV I/O** via the [`csv`](https://crates.io/crates/csv) and [`serde`](https://crates.io/crates/serde) crates.  
- **In-memory state**: `Vec<Employee>` and `GridState` structs.  
- **Persistence**: local file store (JSON or SQLite via [`rusqlite`](https://crates.io/crates/rusqlite)).

### 4.2 GUI Frontend (`box_planner-ui`)  
- Built with the [`iced`](https://crates.io/crates/iced) GUI toolkit for native widgets.  
- **Layout**:  
  - `Column`: left panel + main grid.  
  - `Tabs` for “Employees” / “Skills.”  
  - Custom `Canvas` for 9-box grid.  
- **Drag-and-Drop**: use iced’s input events to implement fluid dragging interactions.

### 4.3 Integration & Packaging  
- Workspace `Cargo.toml` with two members: `core` and `ui`.  
- UI depends on `core` via a path dependency.  
- **Bundling** to installers using [`cargo-bundle`](https://crates.io/crates/cargo-bundle) or [`tauri-bundler`](https://tauri.app/) for Windows `.msi`, macOS `.dmg`, and Linux AppImages.

### 4.4 Testing  
- **Unit tests** in `core` for CSV round-trip, data-model invariants.  
- **Integration tests** launching the UI in “headless” mode and simulating basic flows.  
- **CI/CD** pipeline on GitHub Actions: `cargo test` → `cargo build --release` → bundles.

---

## 5. Data Schema

### Input CSV (rust-csv compatible)  
```csv
Name,Position,PR,Skills,Notes,Box
John Doe,Software Engineer,4,"Skill 1;Skill 2","Excellent team player","High Performer"


**Export CSV Example**
Name,Position,PR Score,Skills,Notes,Box
John Doe,Software Engineer,4,Skill 1;Skill 2,Excellent team player,High Performer

6. Timeline

Phase	Task	Duration
Core Library Setup	Define models, CSV I/O, persistence	2 weeks
UI Prototype	Basic window, tabs, and grid rendering	3 weeks
Drag-and-Drop UX	Implement card/skill interactions	2 weeks
Notes & Themes	Inline notes editor + theming	1 week
Packaging & CI	Bundling + automated builds	1 week
Testing & Polish	Cross-platform validation, bug fixes	1 week

7. Extensibility & Future Enhancements
- Real-time collaboration via WebSockets (using tokio + warp in core).
- Role-based access control (integrate an OAuth2 flow).
- Mobile ports using iced_aw or Tauri WebView.
- Advanced analytics: plug in a plotting crate (e.g., plotters) for dashboard charts.
- Plugin system for custom grid layouts or evaluation workflows.

