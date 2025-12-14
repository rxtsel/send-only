# OnlySend

A minimal desktop application for sending emails from your own domain using **Resend**.
Made with **Rust**, **Tauri**, and **Svelte**.

<img width="1052" height="838" alt="Screenshot 2025-11-27 at 19 29 56" src="https://github.com/user-attachments/assets/7c0f39a7-473c-44f3-a0b8-e12817719156" />

<img width="1241" height="797" alt="Screenshot 2025-11-27 at 19 42 11" src="https://github.com/user-attachments/assets/6c78ea78-3764-4d85-8ac9-6685837f8de3" />

## Download
Visit [releases](https://github.com/rxtsel/only-send/releases) for download.

## Why I built this

I wanted a simple, privacy-friendly tool that lets me send emails from my own domain without needing to maintain a full email hosting setup. Most providers require complicated MX setups, storage, inbox management, spam filtering, and more — even when all you need is **sending**.

OnlySend focuses on one thing:

- [x] **Compose and send emails cleanly under your own domain**

No inbox. No bloat. Just sending.

## How it works

OnlySend uses the **Resend API** to send emails through your verified domain.
You enter your API key, set up your sender identities, and you’re ready to go.

- Add multiple “From” email identities
- Compose messages with rich content
- Attach files
- Send using your own domain
- Local configuration stored securely through Tauri’s store

Everything runs locally — OnlySend does not use any external backend.

## Receiving Email (Optional)

OnlySend is designed **only for sending**.
For receiving, you can combine it with Cloudflare’s free email routing:

- Cloudflare can forward incoming email from your domain… to any inbox you prefer (Gmail, Outlook, etc.)

With Cloudflare forwarding + OnlySend, you cover both sides:

- **Sending** → via OnlySend (Resend API)
- **Receiving** → via Cloudflare Email Routing

No need for a full email hosting provider.

## Features

- - [x] Send polished emails from a clean UI
- - [x] Add CC, BCC, Reply-To, and message headers
- - [x] Attachments support
- - [x] Uses local secure storage for settings
- - [x] Cross-platform via Tauri
- - [x] View a list of sent emails

## Tech Stack

- **Rust** — backend logic + Resend integration
- **Tauri** — desktop app runtime
- **Svelte 5** — front-end UI
- **resend-rs** - Crate for interacting with the Resend API

## Building Locally

To build the application locally:

```
bun tauri build
```

If you encounter issues with the above command (such as errors related to binary stripping), try the following:

```
NO_STRIP=true bun tauri build
```
