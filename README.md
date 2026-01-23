# 💰 Split-Bill Expense Tracker (Rust Microservice)

> **Current Phase**: Backend Core Logic (Completed)
> **Architecture**: RESTful 3-Tier Microservice
> **Status**: Ready for Frontend Integration

## 📖 Project Overview

This project is a high-performance, type-safe **Backend Microservice** designed to handle expense tracking for split bills. It is built using **Rust** for business logic and **Appwrite** for persistent data storage.

Unlike a simple script, this is architected as a robust **API Layer** that keeps the data management decoupled from the user interface, following industry-standard **3-Tier Architecture**.

---

## 🏗️ Architectural Internals

The system is divided into three distinct layers, ensuring scalability and maintainability.

### 1️⃣ Layer 1: The Data Layer (Appwrite) 🗄️

* **Role**: The "Single Source of Truth".
* **Function**: Acts as the Persistent Vault. It strictly supports the schema and ensures data durability.
* **Key Responsibilities**:
  * Generates unique, collision-resistant IDs (`$id`) for every expense.
  * Enforces data types (e.g., preventing text from being stored in the `amount` field).
  * Provides high-availability storage.

### 2️⃣ Layer 2: The Logic Layer (Rust + Axum) 🧠

* **Role**: The "Brain" and "Traffic Controller".
* **Location**: This is the core `src/` directory.
* **Key Components**:
  * **Router (`src/main.rs`)**: The entry point. It maps HTTP requests (like `GET` or `PATCH`) to specific functions. It acts as the gatekeeper, rejecting invalid URLs immediately.
  * **Models (`src/models/`)**: Defines the **Shape of Data**. using Rust `structs`. This guarantees **Type Safety**—if the data doesn't match the struct, the server rejects it before it even crashes.
  * **Handlers (`src/handlers/`)**: The business logic. It translates "Web JSON" into "Rust Objects", performs any necessary calculations or validations, and then talks to the Database.
  * **Clients (`src/clients/`)**: The bridge to external systems (Appwrite). It manages authentication keys and network requests.

### 3️⃣ Layer 3: The Presentation Layer (Pending) 📱

* **Role**: The Interface for Humans.
* **Status**: Currently, the system exposes a **REST API**, which can be consumed by any frontend (Flutter, React, Mobile, or CLI).
* **Future Goal**: A rich UI (like Splitwise) that communicates with this backend.

---

## ⚡ Technical Stack

| Component               | Technology         | Why this choice?                                                                |
| :---------------------- | :----------------- | :------------------------------------------------------------------------------ |
| **Language**      | **Rust 🦀**  | Memory safety, blazing speed, and no garbage collection pauses.                 |
| **Framework**     | **Axum**     | Modular, ergonomic, and built for the modern async web.                         |
| **Database**      | **Appwrite** | Open-source backend-as-a-service providing DB, Auth, and Realtime capabilities. |
| **Serialization** | **Serde**    | The gold standard for fast JSON parsing in Rust.                                |

---

## 🚀 Capabilities (Current Implementation)

The backend currently supports full **CRUD** operations for a "Basic Ledger" system.

| Method           | Endpoint          | Description                                                               | Status         |
| :--------------- | :---------------- | :------------------------------------------------------------------------ | :------------- |
| **POST**   | `/expenses`     | **Create** a new expense entry in the ledger.                       | ✅ Implemented |
| **GET**    | `/expenses`     | **Read** (List) all stored expenses.                                | ✅ Implemented |
| **PATCH**  | `/expenses/:id` | **Update** details of a specific expense (e.g., correct an amount). | ✅ Implemented |
| **DELETE** | `/expenses/:id` | **Delete** an erroneous entry.                                      | ✅ Implemented |

---

## 🗺️ Roadmap & Gap Analysis

To evolve from a **Basic Ledger** to an **Advanced Splitwise-Competitor**, the following upgrades are planned:

### 1. Settlement Algorithm 🧮

* **Current**: Logic only records *who paid*.
* **Gap**: The system doesn't know *who owes whom*.
* **Goal**: Implement a graph minimization algorithm (e.g., Min-Cash-Flow) to simplify debts (e.g., "A owes B $10" instead of "A owes B $50 and B owes A $40").

### 2. Identity & Authentication 🔐

* **Current**: Users are identified by simple strings (`"User_A"`).
* **Gap**: No security; anyone can claim to be anyone.
* **Goal**: Integrate Appwrite Authentication (JWT/Session tokens) to securely map Users to Requests.

### 3. Multi-Payer Support 💸

* **Current**: Only one person can pay the full bill (`paid_by: String`).
* **Gap**: Cannot handle shared payments (e.g., A paid $60, B paid $40 for a $100 dinner).
* **Goal**: Update data models to support `paid_by: Map<User, Amount>`.

### 4. Real-Time Updates 🔔

* **Current**: The frontend must "ask" (poll) for updates.
* **Gap**: Users don't see new bills instantly.
* **Goal**: Implement **WebSockets** (or Appwrite Realtime) to push updates to all group members instantly.

---

## 🛠️ How to Run

1. **Environment Setup**: Ensure `.env` contains your Appwrite credentials (`ENDPOINT`, `PROJECT_ID`, `API_KEY`, `DATABASE_ID`, `COLLECTION_ID`).
2. **Start Server**:
   ```bash
   cargo run
   ```
3. **Test API**: The server listens on `http://127.0.0.1:3000`.

---

*Generated for Expense Tracker Project Evaluation*
