# SpiritVale Overlay

[English](#english) · [ภาษาไทย](#ภาษาไทย)

<a id="english"></a>

## English

A Windows desktop companion for browsing the **SpiritVale** database without leaving your game setup. It is a read-only reference app: it does not inject into, read from, or modify the game client.

### Download

Get the current stable build from the [v1.0.0 release](https://github.com/ftb64/spiritvale-overlay/releases/tag/v1.0.0).

- **Windows installer** — recommended for most players.
- **Portable ZIP** — portable EXE packaged as a ZIP download.
- **SHA-256 checksums** — verify downloaded files if needed.

### Features

- Live catalog sync with a local verified-cache fallback.
- Search across names, stats, drops, crafting, and item details.
- Catalogs for Artifacts, Cards, Consumables, Gems, Maps, Materials, Monsters, Equipment, and Skills.
- Type filters for Cards, Gems, and Equipment.
- Item detail pages with images, stats, sources, and optional pinned selection.
- Four built-in color themes plus a custom theme editor.
- Adjustable window resolutions, animation controls, and English/Thai interface toggle.

### Controls

| Control | Action |
| --- | --- |
| `Alt + E` | Minimize or restore the overlay |
| `−` | Minimize the window |
| `×` | Close the overlay |
| Settings | Change theme, resolution, animation, and refresh data |

You can drag the top header to move the window. Selecting an item returns its details view to the top.

### Data and attribution

Catalog information and source links are synced from [SpiritVale.info](https://www.spiritvale.info/). SpiritVale Overlay caches the last verified response locally so the reference remains available when a sync cannot finish.

### Development

Requirements: Node.js, Rust, and the Windows prerequisites for [Tauri v2](https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run tauri dev
```

Build a production installer:

```bash
npm run tauri build
```

<a id="ภาษาไทย"></a>

## ภาษาไทย

โปรแกรมคู่มือ SpiritVale สำหรับ Windows ช่วยให้คุณเปิดดูฐานข้อมูลของ **SpiritVale** ได้โดยไม่ต้องออกจากหน้าจอเกม เป็นโปรแกรมอ้างอิงแบบอ่านอย่างเดียว จึงไม่ฉีดโค้ด อ่านข้อมูล หรือแก้ไขตัวเกม

### ดาวน์โหลด

ดาวน์โหลดเวอร์ชันเสถียรล่าสุดได้จาก [v1.0.0 release](https://github.com/ftb64/spiritvale-overlay/releases/tag/v1.0.0)

- **Windows installer** — แนะนำสำหรับผู้เล่นส่วนใหญ่
- **Portable ZIP** — ไฟล์ Portable EXE ในรูปแบบ ZIP
- **SHA-256 checksums** — ใช้ตรวจสอบความถูกต้องของไฟล์ที่ดาวน์โหลด

### ความสามารถ

- ซิงก์แค็ตตาล็อกแบบสด และใช้แคชที่ตรวจสอบแล้วเมื่อซิงก์ไม่สำเร็จ
- ค้นหาจากชื่อ ค่าสถานะ ดรอป การคราฟต์ และรายละเอียดไอเทม
- มีแค็ตตาล็อก Artifacts, Cards, Consumables, Gems, Maps, Materials, Monsters, Equipment และ Skills
- กรองประเภทของ Cards, Gems และ Equipment ได้
- หน้ารายละเอียดไอเทมมีรูปภาพ ค่าสถานะ แหล่งข้อมูล และการปักหมุดรายการ
- มีธีมสำเร็จรูป 4 แบบ พร้อมตัวแก้ไขธีมแบบกำหนดเอง
- ปรับขนาดหน้าต่าง การเคลื่อนไหว และเปลี่ยนภาษาอังกฤษ/ไทยได้

### การควบคุม

| ปุ่ม | การทำงาน |
| --- | --- |
| `Alt + E` | ย่อหรือเรียกหน้าต่าง Overlay กลับมา |
| `−` | ย่อหน้าต่าง |
| `×` | ปิดหน้าต่าง |
| Settings | เปลี่ยนธีม ความละเอียด การเคลื่อนไหว และรีเฟรชข้อมูล |

ลากส่วนหัวด้านบนเพื่อย้ายหน้าต่างได้ เมื่อเลือกรายการ หน้ารายละเอียดจะเลื่อนกลับไปด้านบนเสมอ

### ข้อมูลและการอ้างอิง

ข้อมูลแค็ตตาล็อกและลิงก์แหล่งข้อมูลซิงก์จาก [SpiritVale.info](https://www.spiritvale.info/) โปรแกรมจะเก็บแคชของข้อมูลที่ตรวจสอบแล้วไว้ในเครื่อง เพื่อให้ยังค้นดูข้อมูลได้หากการซิงก์ไม่เสร็จ

### สำหรับผู้พัฒนา

ต้องมี Node.js, Rust และข้อกำหนด Windows ของ [Tauri v2](https://v2.tauri.app/start/prerequisites/)

```bash
npm install
npm run tauri dev
```

สร้างตัวติดตั้งสำหรับใช้งานจริง:

```bash
npm run tauri build
```

## License / ใบอนุญาต

MIT. See [LICENSE](LICENSE).
