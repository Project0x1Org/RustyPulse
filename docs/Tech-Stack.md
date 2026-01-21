# Uptime Monitor: Prototype Tech Stack

This document outlines the minimalist, high-performance tech stack selected for the initial prototype.
The focus is on low resource consumption, zero-bloat, and a single-binary output.

---

## 1. Definition of the Tech Stack

A **Tech Stack** is the underlying ecosystem of tools used to build and run an application. For this project, 
we have selected a **Native Rust Stack**. 
By avoiding web technologies (like Electron or Tauri), we reduce memory usage by up to 90% and ensure the application remains
responsive even under heavy network load.

---

## 2. The Core Components

| Component            | Technology            | Category               | Role & Importance                                                                                                                                              |
|:-------------------- |:--------------------- |:---------------------- |:-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Language**         | **Rust**              | System Language        | Provides the speed of C++ with memory safety. It ensures the monitor is stable and never crashes due to memory leaks.                                          |
| **GUI Framework**    | **egui** (via eframe) | Native UI              | An "Immediate Mode" GUI. It renders directly to the GPU. Since there is no HTML/CSS parsing, the UI is incredibly fast and the code remains simple and linear. |
| **Async Runtime**    | **Tokio**             | Backend Infrastructure | The "Engine" that allows the app to handle multiple pings at once without the user interface freezing or stuttering.                                           |
| **Network Protocol** | **surge-ping**        | Network Logic          | A lightweight ICMP (Ping) implementation. It allows us to send raw packets directly to a URL to measure latency with microsecond precision.                    |

---

## 3. Justification & Roles

### **Why Rust?**

Rust allows us to build a "bare-metal" application. In an uptime monitor, the monitoring tool should not consume significant
system resources. Rust ensures that almost 100% of the CPU is dedicated to network analysis rather than background overhead.

### **Why egui (No HTML/CSS)?**

Most modern GUIs use a web-view (HTML/CSS), which requires a massive browser engine to run in the background. 
**egui** eliminates this. It is a pure Rust library that draws shapes directly on the screen. 

* **Role:** Handles the dashboard layout, URL input fields, and real-time status text.
* **Importance:** Keeps the final executable size tiny, and the RAM usage minimal.

### **Why surge-ping & Tokio?**

Pinging a URL involves a "wait period" for the signal to return. 

* **Role:** **Tokio** manages the waiting periods in the background, while **surge-ping** handles the actual handshakes
  with the target URL.
* **Importance:** This allows the prototype to monitor 1, 10, or 100 URLs simultaneously without any drop in performance.

---

## 4. Summary of Dependencies

To initialize this stack, the following will be included in the `Cargo.toml`:

* `eframe` (The framework wrapper for egui)
* `tokio` (With `rt` and `macros` features)
* `surge-ping` (For the core functionality)
