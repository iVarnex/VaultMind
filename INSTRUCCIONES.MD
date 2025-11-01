# 📘 Instrucciones del Proyecto: VaultMind  

## 🧠 Descripción General
VaultMind es una aplicación de escritorio multiplataforma con **Tauri + React + TypeScript**, centrada en la **seguridad y privacidad local**.  
Permite gestionar contraseñas, notas y archivos cifrados sin depender de la nube.  

---

## 🎯 Objetivos
1. Crear una app moderna y privada con cifrado AES-256-GCM + Argon2id.  
2. Diseñar una interfaz limpia (modo claro/oscuro).  
3. Integrar SQLite/SQLCipher como almacenamiento local.  
4. Incluir módulos: Vault · Secure Notes · Secure Files · Settings.  
5. Implementar cierre automático por inactividad.  

---

## ⚙️ Requisitos Técnicos
- Frontend : React + TypeScript  
- Backend : Rust (Tauri Core)  
- Estilos : TailwindCSS + Shadcn/UI  

    npm install  
    npm run tauri dev  
    npm run tauri build  

---

## 🧱 Estructura
    VaultMind/
    ├── src/
    │   ├── components/
    │   ├── pages/
    │   ├── styles/
    │   └── main.tsx
    ├── src-tauri/
    │   └── main.rs
    └── package.json  

---

## 🎨 Paletas de Colores  

### 🌑 Modo Oscuro
|Elemento|Color|HEX|
|-|-|-|
|Fondo|Azul neón oscuro|#0E141B|
|Secundario|Gris grafito|#1A1F25|
|Primario|Azul eléctrico|#3B82F6|
|Texto|Blanco suave|#E5E7EB|
|Éxito|Verde menta|#10B981|
|Advertencia|Amarillo dorado|#F59E0B|
|Error|Rojo coral|#EF4444|

### ☀️ Modo Claro
|Elemento|Color|HEX|
|-|-|-|
|Fondo|Blanco neutro|#F9FAFB|
|Secundario|Gris pálido|#F3F4F6|
|Primario|Azul pastel|#93C5FD|
|Texto|Gris carbón|#111827|
|Éxito|Verde claro|#6EE7B7|
|Advertencia|Amarillo suave|#FCD34D|
|Error|Rosa coral claro|#FCA5A5|

---

## 🎨 Variables CSS (Tailwind)
Agregar en **src/styles/globals.css**  

    :root {
      --color-bg: #F9FAFB;
      --color-text: #111827;
      --color-primary: #93C5FD;
      --color-success: #6EE7B7;
      --color-warning: #FCD34D;
      --color-error: #FCA5A5;
    }

    .dark {
      --color-bg: #0E141B;
      --color-text: #E5E7EB;
      --color-primary: #3B82F6;
      --color-success: #10B981;
      --color-warning: #F59E0B;
      --color-error: #EF4444;
    }

---

## 🧩 Configuración Tailwind
En `tailwind.config.js`  

    export default {
      darkMode: 'class',
      content: ['./index.html', './src/**/*.{ts,tsx}'],
      theme: { extend: {} },
      plugins: [],
    }

En `App.tsx`  

    useEffect(() => {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      document.documentElement.classList.toggle('dark', prefersDark);
    }, []);

---