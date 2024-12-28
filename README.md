
# Proof of Existence Pallet

## Overview
The **Proof of Existence Pallet** is a Rust-based blockchain module that provides a system for verifying and managing ownership of digital content. This pallet allows users to create, retrieve, and revoke claims on content stored in a blockchain ledger, ensuring transparency, immutability, and proof of ownership.

---

## Features

### 1. **Core Functionality**
- **Create Claims**: Users can register ownership of content by creating claims.
- **Retrieve Claims**: Retrieve ownership details of specific content.
- **Revoke Claims**: Owners can revoke their claims if necessary.

### 2. **Dispatch Logic**
- Implements a unified interface to handle all claim-related actions (create, retrieve, and revoke).
- Supports dynamic routing of function calls for efficient execution.

### 3. **Runtime Integration**
- Seamlessly integrates with the blockchain runtime, enabling claim operations through block execution.
- Supports configuration for account IDs and content types.

### 4. **Block Execution**
- Claims can be managed dynamically within blocks, showcasing flexible transaction handling.
- Supports extrinsics for creating, managing, and revoking claims via runtime calls.

### 5. **Testing**
- Comprehensive unit tests to ensure the correctness of claim creation, retrieval, and revocation.
- Includes scenarios for invalid operations (e.g., duplicate claims, unauthorized revocation).

---

## How It Works

### Claims
A **claim** is a mapping of `Content` to an `AccountId`, representing ownership of the content by the account.

### Dispatch Calls
The pallet provides an interface for the following actions:
- **CreateClaim**: Register ownership of content.
- **RevokeClaim**: Remove ownership of content.

---

## How to Run

### Prerequisites
- Install Rust: [Rust Installation](https://www.rust-lang.org/tools/install)

### Steps
1. **Clone the Repository**:
   ```
   git clone <repository_url>
   cd ProofOfExistencePallet
   ```

2. **Build and Run**:
   ```bash
   cargo build
   cargo run
   ```

3. **Run Tests**:
   ```
   cargo test
   ```

---

## Example Output

### Running the Program
The program demonstrates claim creation and revocation:

```yaml
Runtime State: Runtime { proof: ProofOfExistence { claims: {} } }
```

---

## Highlights

### **Modularity**
- Decoupled implementation for `ProofOfExistence` ensures clarity and reusability.

### **Flexibility**
- Uses generics for flexible configuration of `AccountId` and `Content`.

### **Scalability**
- Easily integrates with other blockchain modules and pallets.

### **Immutability**
- Ensures that claims, once created, cannot be tampered with, offering a robust proof of ownership.

---

## Features Under Development
- Integration with additional pallets for extended functionality.
- Advanced error handling for better user feedback.
