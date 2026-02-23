# Contributing to ESTV

Thank you for your interest in contributing to ESTV! This document provides guidelines and steps for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How to Contribute

### Reporting Bugs

1. Check existing [Issues](https://github.com/ESTVLAB/ESTV/issues) to avoid duplicates
2. Use the bug report template
3. Provide detailed reproduction steps
4. Include relevant logs and screenshots

### Suggesting Features

1. Open a [Feature Request](https://github.com/ESTVLAB/ESTV/issues/new?template=feature_request.md)
2. Describe the problem you're trying to solve
3. Explain your proposed solution
4. Consider alternatives you've thought about

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Write or update tests as needed
5. Ensure all tests pass
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to your branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(token): add staking functionality
fix(api): resolve rate limiting issue
docs(readme): update installation instructions
```

## Development Setup

### Prerequisites

- Rust 1.70+
- Solana CLI 1.16+
- Anchor Framework 0.29+
- Node.js 18+

### Local Development

```bash
# Clone the repository
git clone https://github.com/ESTVLAB/ESTV.git
cd ESTV

# Install dependencies
npm install

# Build the project
anchor build

# Run tests
anchor test
```

## Project Structure

```
ESTV/
├── programs/           # Solana programs (smart contracts)
│   └── estv/
│       └── src/
├── tests/              # Integration tests
├── scripts/            # Deployment and utility scripts
├── docs/               # Documentation
├── assets/             # Brand assets and images
└── audits/             # Security audit reports
```

## Style Guide

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting

### TypeScript/JavaScript

- Use ESLint with our configuration
- Use Prettier for formatting

## Review Process

1. All PRs require at least one review
2. CI checks must pass
3. Documentation must be updated if needed
4. Breaking changes require discussion

## Getting Help

- [GitHub Discussions](https://github.com/ESTVLAB/ESTV/discussions)
- [Telegram Community](https://t.me/estv_esportstv)
- Email: dev@estvlabs.com

## Recognition

Contributors will be acknowledged in:
- Our README
- Release notes
- Community spotlights

Thank you for contributing to ESTV!
