# Overwatch Explorer

Overwatch Explorer is a high-performance, Rust-powered folder management tool designed to track recent activity and navigate complex directory trees with speed and precision.

---

### 🚀 Getting Started

#### **Prerequisites**

* **Rust Toolchain**: Ensure you have Rust **1.74 or newer** installed. (Run `rustup update` if you are on an older version).
* **pnpm**: This project uses **pnpm** for frontend dependency management due to its speed and disk-space efficiency.
* **Build Tools**: On Windows, you must have the **C++ Build Tools** installed via the Visual Studio Installer.

#### **Installation & Launch**

1. **Install Dependencies**:
```bash
pnpm install

```


2. **Run in Development Mode**:
```bash
pnpm tauri dev

```



---

### 🛠 How It Works

The app is built using **Tauri v2**, which bridges a lightning-fast **Rust backend** with a modern **HTML/Tailwind CSS frontend**.

#### **1. The Rust Core (The "Brain")**

* **Recursive Scanning**: When a folder is dropped, the Rust backend performs a deep recursive scan of the directory, capturing file metadata like size, extension, and modification timestamps.
* **IPC Bridge**: The frontend communicates with Rust using **Invokes**. Rust handles the heavy lifting of disk I/O, ensuring the interface never freezes.
* **Native Integration**: It uses native Windows commands to reveal files directly in File Explorer.

#### **2. The Frontend (The "Interface")**

* **Lazy Loading**: In "Tree View," child folders are only rendered in the DOM when you click to expand them. This allows the app to handle directories with tens of thousands of files effortlessly.
* **Virtual Sorting**: In "Recent View," the app flattens the directory tree and sorts the top results (25, 50, or 100) by the most recent modification time.
* **Persistent State**: Your workspace is automatically saved to `localStorage`, so your watched folders remain in the sidebar even after restarting the app.

---

### 📖 Usage Guide

#### **Adding Sources**

* **Drag & Drop**: Simply drag any folder from your computer and drop it into the sidebar or the central drop zone.
* **Open List**: Use the **Open JSON** button to load a previously saved workspace configuration.

#### **Navigating Views**

* **Recent Activity (Default)**: Shows a flat list of the newest updates across the entire source.
* **Tree View**: A classic hierarchical view of your folders. Use the **Expand All** button for a quick overview of the structure.

#### **Interaction Features**

* **Visual Status**:
* 🟢 **Glowing Green**: Updated within the last hour.
* 🟠 **Faded Orange**: No updates in over 30 days.


* **Column Resizing**: Hover over the vertical lines between column headers and drag to adjust width. A blue "Ghost Line" will guide your resize.
* **Reveal & Copy**: Click the file path in Recent View to open that folder in Windows Explorer, or click **Copy** to grab the full path for your clipboard.

---

### 🛡 Permissions & Security

This app follows the **Tauri v2 Security Model**. It explicitly requires the following capabilities to be defined in `src-tauri/capabilities/default.json`:

* `dialog:allow-save` & `dialog:allow-open`
* `fs:allow-read-text-file` & `fs:allow-write-text-file`
* `dragDropEnabled` set to `true` in `tauri.conf.json`

---

**Would you like me to generate a `package.json` file with all the metadata and scripts needed for this project?**