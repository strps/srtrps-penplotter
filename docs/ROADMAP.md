# Project Roadmap

This document outlines the planned features, milestones, and timeline for the PenPlotter project.

🔌 Next Milestones

Here’s a logical roadmap you could follow:

Phase 1: Geometry Migration (now)

✅ Replace manual flattening + transforms with kurbo
✅ Keep the same API for /convert

Phase 2: Plotter Transform & Offsets

✅ Implement device coordinate mapping cleanly with a PlotterTransform struct
✅ Support arbitrary SVG reference points (bottom-left, center, etc.)

Phase 3: Fill Patterns

⚙️ Use kurbo paths + geo clipping to create hatch fills

Phase 4: Local G-code Execution

⚙️ Add serial streaming using tokio-serial
⚙️ Basic GRBL protocol: $, ?, ~, !

Phase 5: WebSocket + Dashboard

🌐 Live status updates + G-code progress
🌐 Manual controls (jogging, homing)

Phase 6: Frontend

💻 Simple SPA served from Axum or static directory
💻 Real-time view of toolpath + controls

## Current Status

The project is currently in **Phase 1: Geometry Migration**, focusing on integrating kurbo for geometry processing while maintaining API compatibility.

## Long-term Vision

- **Hardware Agnosticism**: Support multiple device backends (e.g., GRBL, Marlin, FluidNC) to broaden compatibility.
- **Real-time Collaboration**: Enable WebSocket-based live G-code streaming and remote control for distributed plotting setups.
- **Advanced Visualization**: Integrate preview services with Canvas-based toolpath rendering and collision detection.
- **Scalability**: Implement local caching, job queues, and multi-plotter support for production environments.
- **User Experience**: Develop a polished frontend UI with authentication, settings persistence, and mobile responsiveness.
- **Community Growth**: Foster contributions through clear guidelines, automated testing, and modular extensibility for custom algorithms or devices.

---

## Summary

This roadmap prioritizes a phased, iterative approach to evolve the PenPlotter from a basic converter into a comprehensive plotting platform. Each phase builds on the previous, ensuring stability and extensibility.
