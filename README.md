# RustyGuard

Intrusion Detection System (IDS) application designed in Rust and SvelteKit packaged with Tauri.

## Project Summary

RustyGuard is an Intrusion Detection System that leverages the efficiency of the Rust programming language to monitor and filter malicious network traffic in real-time. It combines a high-performance backend with an intuitive SvelteKit based frontend, enabling users to easily configure the system via a Tauri packaged desktop application.

## Goals

- **Reliability**: Develop a robust system to detect an array of network threats.
- **User Experience**: Craft a user-friendly interface for tailored settings and observations.
- **Scalability**: Lay a foundation for future enhancements, including machine learning capabilities.

## Roadmap

### Planning

- [x] Conduct research on existing IDS models.
- [ ] Outline project scope with precise requirements.
- [x] Selected appropriate Rust crates for network functionality (`pnet & ipnet`)

### Development

#### Backend

- [ ] Establish packet capturing mechanics, echoing packets to terminal for verification.
- [ ] Forge an analyzer to scrutinize traffic versus blacklist specifications.
- [ ] Implement data structures for managing blacklisted entities.
- [ ] Develop capabilities for real-time traffic blocking and alarms.
- [ ] Introduce testing suites for all modules.

#### Frontend

- [ ] Generate an accessible interface for configuration and alert oversight.
- [ ] Bind frontend systems with Rust backend.
- [ ] Facilitate personalizable blacklist parameters through UI.
- [ ] Embed real-time traffic visualizations.

#### Integration

- [ ] Assure fluent interaction between frontend and backend facets.
- [ ] Conduct comprehensive system evaluations.

### Testing

- [ ] Execute rigorous unit testing; debug as necessary.
- [ ] Implement integration tests within the application scope.
- [ ] Perform full-scale end-to-end testing under simulated attacks.
- [ ] Collect and analyze feedback from a beta testing group.
- [ ] Polish based on insights and test findings.

### Deployment

- [ ] Package the application for MacOS, Linux, and Windows, including icons and assets.
- [ ] Document the setup, dependencies, configuration guides, and use-cases of RustyGuard.
- [ ] Release the first version on GitHub under the releases section.

### Credits and References

Shout out to the book ["Network Programming with Rust: Build fast and resilient network servers and clients by leveraging Rust's memory-safety and concurrency features" by Abhishek Chanda](https://www.packtpub.com/product/network-programming-with-rust/9781788624893) for the higher level understanding of networking details and rust implementation examples.

Also checkout the [Rust Programming Language](https://www.rust-lang.org/), [SvelteKit](https://kit.svelte.dev/), and [Tauri](https://tauri.studio/). Their tooling and documentation made this project possible.

Lastly, thanks to [SkeletonUI](https://www.skeleton.dev/) for providing execelent CSS styles and components for the frontend.
