# Torrust Index (Meta-Repository)

This is the meta-repository for the Torrust Index project, which consists of two separate repositories:

- [Torrust Index Backend](https://github.com/torrust/torrust-index-backend): A Rust backend that serves as an index for torrent files.
- [Torrust Index Frontend](https://github.com/torrust/torrust-index-frontend): A Vue frontend that connects to the backend and provides a user interface for searching and browsing torrent files.

This meta-repository:

- Provides an overview of the Torrust Index project and explains the relationship between the backend and frontend repositories.
- Provides a context for issues and discussions that are not specific to either the backend or frontend repositories.

![Torrust Architecture](https://raw.githubusercontent.com/torrust/.github/main/img/torrust-architecture.webp)

## About Torrust Index

Torrust Index is a project that aims to provide a simple and efficient way to search for and browse torrent files. The project consists of two main components: the backend and the frontend.

The backend is built with Rust and provides a REST API for querying a database of torrent files. It uses the `actix-web` framework for handling HTTP requests and `diesel` for interacting with the database.

The frontend is built with Vue.js and provides a modern and responsive user interface for searching and browsing the index.

## Repository Structure

This meta-repository does not contain any code but serves as a central point for accessing the two main repositories that make up the Torrust Index project.

The [Torrust Index Backend](https://github.com/torrust/torrust-index-backend) repository contains the source code for the backend. It includes a `README.md` file with instructions for building and running the backend.

The [Torrust Index Frontend](https://github.com/torrust/torrust-index-frontend) repository contains the source code for the frontend. It includes a `README.md` file with instructions for building and running the frontend.

## Contributing

We welcome contributions from the community!

How can you contribute?

- Bug reports and feature requests.
- Code contributions. You can start by looking at the issues labeled `good first issues`.
- Documentation improvements. Check the documentation for typos, errors, or missing information.
- Participation in the community. You can help by answering questions in the [discussions](https://github.com/torrust/torrust-index/discussions).

## License

The project is licensed under a dual license. See [COPYRIGHT](./COPYRIGHT). See the `LICENSE` file in each of the two main repositories for details.
