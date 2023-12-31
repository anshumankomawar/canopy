# Canopy

Canopy is a versatile and easy-to-use service for creating comprehensive public API documentation for your services. With Docs, you can streamline the process of documenting your APIs, making it easier for developers to understand and utilize your services. This README provides an overview of the project and its core Rust modules: auth, database, api, and search.

## Features

- **Effortless Documentation**: Simplify the task of documenting your API endpoints, data models, and usage examples.
- **Customizable Themes**: Tailor your API documentation to match your brand or project's style.
- **Interactive API Explorer**: Allow users to interact with your API directly from the documentation.
- **User Authentication**: Secure your documentation and restrict access with user authentication.
- **Powerful Search**: Enable users to quickly find the information they need with our advanced search functionality.

## Modules

Docs is built using Rust, and the project is organized into several modules, each serving a specific purpose:

### 1. Auth Module

The `auth` module is responsible for handling user authentication and access control to your API documentation. It ensures that only authorized users can view and edit documentation, providing an added layer of security.

#### Features:

- User registration and login.
- Role-based access control.
- Password hashing and encryption for user data protection.

### 2. Database Module

The `database` module manages the storage and retrieval of documentation data, user profiles, and access control settings. It is crucial for the persistence and reliability of your documentation service.

#### Features:

- Storage of API documentation content.
- User profile management.
- Integration with various database systems.

### 3. API Module

The `api` module forms the core of the Docs service. It allows you to create, update, and publish API documentation. This module includes functionalities for documenting API endpoints, describing request and response formats, and generating interactive documentation.

#### Features:

- API endpoint documentation.
- Request and response examples.
- Swagger/OpenAPI compatibility.

### 4. Search Module

The `search` module enhances the user experience by providing robust search capabilities within your documentation. Users can quickly find relevant endpoints, data models, or usage examples using keywords or advanced search queries.

#### Features:

- Full-text search.
- Advanced search filters.
- Search indexing for optimal performance.

## Getting Started

To get started with Docs, follow these steps:

1. Clone the repository: `git clone https://github.com/anshumankomawar/canopy.git`
2. Install Rust and dependencies.
3. Set up and configure the `auth`, `database`, `api`, and `search` modules as needed.
4. Build and run the service.
5. Access the documentation portal in your web browser.

For detailed installation and configuration instructions, refer to the respective module README files.

### Github Guidlines

Add: Create a capability (e.g., feature, test, dependency).
Cut: Remove a capability (e.g., feature, test, dependency).
Fix: Fix an issue (e.g., bug, typo, accident, misstatement).
Bump: Increase the version of something (e.g., dependency).
Make: Change the build process, or tooling, or infra.
Start: Begin doing something; (e.g., create a feature flag).
Stop: End doing something; (e.g., remove a feature flag).
Refactor: A code change that MUST be just a refactoring.
Reformat: Refactor of formatting, (e.g., omit whitespace).
Optimise: Refactor of performance, (e.g., speed up code).
Document: Refactor of documentation, (e.g., help files).

Happy documenting with Canopy! 🚀
