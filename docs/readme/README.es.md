https://github.com/user-attachments/assets/c827f059-cfd7-4f6a-bed3-8b00411a7220

<p align="center">
  <strong>Una alternativa gratuita y de código abierto a Stardock Fences para Windows 11.</strong><br>
  Agrupa archivos en paneles de cristal, cambia de proyecto con pestañas y deja que tu agente de IA configure la distribución, la apariencia y las reglas de organización con la CLI integrada.
</p>

<p align="center">
  <a href="https://pecofence.jiang.jp/es/"><strong>Sitio web</strong></a>
  &nbsp;·&nbsp; <a href="#descarga-pecofence"><strong>Descarga PecoFence →</strong></a>
  &nbsp;·&nbsp; <a href="#tu-escritorio-configurado-por-tu-ia"><strong>IA + CLI</strong></a>
  &nbsp;·&nbsp; <a href="#míralo-en-acción">Míralo en acción</a>
  &nbsp;·&nbsp; <a href="../README.md">Documentación</a>
</p>

<p align="center">
  <a href="../../README.md">English</a>
  &nbsp;·&nbsp; <a href="README.zh-CN.md">简体中文</a>
  &nbsp;·&nbsp; <a href="README.zh-TW.md">繁體中文</a>
  &nbsp;·&nbsp; <a href="README.ja.md">日本語</a>
  &nbsp;·&nbsp; <a href="README.ko.md">한국어</a>
  &nbsp;·&nbsp; <a href="README.de.md">Deutsch</a>
  &nbsp;·&nbsp; <a href="README.fr.md">Français</a>
  &nbsp;·&nbsp; <strong>Español</strong>
  &nbsp;·&nbsp; <a href="README.pt-BR.md">Português (Brasil)</a>
  &nbsp;·&nbsp; <a href="README.ru.md">Русский</a>
  &nbsp;·&nbsp; <a href="README.ar.md">العربية</a>
</p>

---

## La integración definitiva de escritorio estático y estanterías de pósters

![La integración definitiva de escritorio estático y estanterías de pósters](../assets/desktop-poster-shelves.png)

### 🧩 La pieza que faltaba: BoxArt Studio
PecoFence aporta la cuadrícula rígida y los contenedores de póster 2:3 sin huecos, pero Windows sigue generando iconos cuadrados estándar.
Para conseguir el aspecto exacto de estantería física de videojuegos que se ve en la captura, necesitas **[BoxArt Studio](https://github.com/fox10881/BoxArt-Studio)**. BoxArt Studio es la herramienta complementaria esencial que convierte por lotes tus accesos directos de juegos y carpetas de Windows en elegantes carátulas 2:3 de alta resolución.

### 📸 Instantáneas de distribución portátiles

Las instantáneas de distribución ahora son copias de seguridad totalmente portátiles: renombra un archivo guardado directamente en la lista de instantáneas y, si quieres, empaqueta el fondo de pantalla actual dentro del propio archivo. La distribución se restaura entonces al píxel en cualquier PC — incluso en uno que nunca haya visto tu fondo.

### 🧊 Estantería 3D inclinada (experimental)

Cizalla cualquier grupo hasta convertirlo en un paralelogramo perfecto para que sus filas sigan las baldas de un fondo de pantalla en ángulo: arrastra los tiradores verticales del borde superior de un lado para inclinar el grupo, y los iconos se escalan progresivamente hacia el borde más cercano y bajo — ajustable con el deslizador **Factor de profundidad** en la configuración del grupo.

> **⚠️ Problema conocido — se agradece ayuda.** El movimiento del ratón, la alineación y la detección de clics dentro del paralelogramo inclinado funcionan perfectamente. Sin embargo, con un fondo de pantalla de perspectiva 3D, la primera fila se alinea perfectamente con la balda, pero las siguientes no, debido a la perspectiva del punto de fuga del fondo. Si algún desarrollador sabe resolver perfectamente esta matemática de perspectiva en varias filas, ¡los pull requests son más que bienvenidos!

### 🖼️ Fondos de pantalla 2D listos para usar

El repositorio incluye un paquete seleccionado de fondos de estanterías en vista frontal plana en [`assets/wallpapers/`](../../assets/wallpapers/), que se alinean a la perfección con los grupos planos sin inclinar: elige uno, coloca tus baldas y listo.

> **💡 Consejo profesional:** Hemos incluido fondos 2D listos para usar que se alinean perfectamente con los grupos planos. Para una configuración espectacular, te recomendamos pasar estas imágenes 2D por un generador de vídeo con IA para animarlas y aplicarlas a tu escritorio con [Lively Wallpaper](https://github.com/rocksdanister/lively).

<a id="pídele-a-tu-ia-que-lo-organice"></a>

## Tu escritorio, configurado por tu IA

**CLI incluida. Lista para tu agente de IA.**

Dile a tu agente de IA cómo quieres usar tu escritorio. Con `pecofence-cli`, incluida en la aplicación, Claude Code, Codex y Cursor pueden leer tu configuración actual y aplicar cambios directamente en PecoFence.

- **Configura con tus propias palabras.** Cambia temas, transparencia, tamaño de los iconos y ajustes globales, o modifica todos los grupos a la vez.
- **Organiza una vez y mantén el orden.** Crea grupos por proyecto, distribuye los iconos y agrega reglas que clasifiquen los archivos nuevos automáticamente.
- **Guarda tu configuración favorita.** Usa instantáneas para la distribución de los grupos y la exportación e importación de configuración para ajustes, reglas y distribuciones.

**Pruébalo con tu agente de IA**

Abre PecoFence y pega esta petición en tu agente de programación con IA:

> Configura mi escritorio con pecofence-cli. Primero lee la salida de pecofence-cli skill y pecofence-cli describe; luego revisa mis ajustes y grupos actuales. Haz una copia de mi configuración antes de cambiarla. Activa el modo oscuro y haz todos los grupos más transparentes.

La CLI está incluida. La edición de Microsoft Store agrega `pecofence-cli` al PATH. Con el ZIP portátil, indica a tu agente la ruta completa de `pecofence-cli.exe`.

<details>
<summary><strong>Conversación de ejemplo con un agente de programación</strong></summary>

> Pon los PDF de mi escritorio en un grupo Docs y clasifica allí también los PDF nuevos. Activa el modo oscuro y haz los grupos más transparentes.

```powershell
pecofence-cli config export "$env:USERPROFILE\pecofence-before-ai.json"
pecofence-cli snapshot save before-cleanup
pecofence-cli fence create --title Docs
pecofence-cli rule add --name PDFs --ext pdf --to Docs --index 0
pecofence-cli rule apply
pecofence-cli settings set theme dark
pecofence-cli fence set --all opacity clear
```

</details>

Para agentes y scripts: `describe` ofrece el catálogo de comandos y los esquemas JSON; `skill`, la guía del agente. Los resultados JSON indican qué cambió y los errores estructurados de la aplicación ayudan al agente a elegir el siguiente paso.

[Primeros pasos con la CLI →](../CLI.md#start-with-your-ai-agent)

## Dale un lugar a cada cosa

<p align="center">
  <img src="../assets/hero-es.png" alt="PecoFence — Carpetas de proyecto, un PDF real y diseños originales en grupos nativos de Liquid Glass." width="1280">
</p>

## Míralo en acción

### Una ventana. Varios espacios de trabajo.

Mantén juntos los grupos relacionados como pestañas. Pasa de Project a Ideas con un clic y,
cuando necesites más espacio, arrastra una pestaña fuera para convertirla en su propio grupo.

![Dos grupos se unen en pestañas, se cambia entre Project e Ideas y luego una pestaña se separa en un grupo independiente.](../assets/tabs.gif)

### Tu escritorio, a un atajo de distancia.

Presiona **Ctrl + Alt + Espacio** y la vista rápida trae tus grupos por encima de la aplicación actual.
Toma lo que necesites y presiona **Esc** para volver.

![La vista rápida muestra los grupos del escritorio sobre una aplicación; un clic fuera o abrir un archivo devuelve a la aplicación.](../assets/peek.gif)

<sub>Animaciones del <a href="https://pecofence.jiang.jp/es/manual/">manual</a>. Los GIF se repiten automáticamente.</sub>

## Pequeños detalles que mejoran el día a día

| Experiencia | Qué obtienes |
| :--- | :--- |
| **IA + CLI** | `pecofence-cli` — **Configura con tus propias palabras.** Cambia temas, transparencia, tamaño de los iconos y ajustes globales, o modifica todos los grupos a la vez. |
| **Menos tiempo ordenando** | Reglas por tipo de archivo, extensión, nombre, comodines, destino del acceso directo, hora y tamaño. Los archivos nuevos encuentran su grupo solos. |
| **Cristal a la medida de tu escritorio** | Temas Fluent y Liquid Glass, modos claro y oscuro, colores por grupo, opacidad y tinte de iconos. |
| **Archivos como siempre** | Menús contextuales del Explorador de archivos, arrastrar y soltar, copiar y pegar, selección múltiple, miniaturas y vistas de iconos, lista y detalles. |
| **Ten tus carpetas a mano** | Coloca una carpeta en vivo sobre el escritorio. Navega por sus subcarpetas y ve los cambios al momento. |
| **Espacio cuando lo necesitas** | Contrae un grupo hasta su título. Pasa el cursor para expandirlo. Bloquea la distribución que te gusta. Haz doble clic en el escritorio para ocultar los grupos. Vuelve a hacer doble clic para recuperarlos. |
| **Siempre puedes volver atrás** | Instantáneas de distribución, copias de seguridad diarias, importación y exportación de la configuración e intercambio entre pantallas. |
| **Estantería 3D inclinada (experimental)** | Cizalla un grupo hasta formar un paralelogramo para que sus filas sigan las baldas de un fondo en ángulo, con escalado de profundidad de los iconos hacia el borde cercano. |
| **Huella mínima** | Una aplicación nativa escrita en Rust que en reposo ocupa unos 40 MB de memoria (según el Administrador de tareas). El panel de Configuración en WebView2 se carga solo cuando hace falta. |

Las reglas de organización automática dejan los archivos en su ubicación original. Los movimientos
que inicias tú funcionan igual que en el Explorador de archivos.

[Explora la lista completa de funciones →](../FEATURES.md)

## Descarga PecoFence

<a href="https://apps.microsoft.com/detail/9MV6WG3XNWSX?mode=direct"><img src="https://get.microsoft.com/images/es%20dark.svg" alt="Consíguelo en Microsoft Store" width="200"></a>

La versión de Microsoft Store está firmada por Microsoft, se actualiza sola y nunca muestra el aviso de SmartScreen. La misma aplicación también está disponible como:

- **ZIP portátil**: descarga `pecofence-v<versión>-x64-portable.zip` desde la página de **Releases** de este repositorio, extrae el **ZIP completo** en una carpeta y ejecuta `pecofence.exe`.
- **Instalador**: `pecofence-v<versión>-x64-setup.exe`, en la misma página, instala PecoFence para tu usuario de Windows, con acceso en el menú Inicio y desinstalador.
- **winget**: `winget install DayuanJiang.PecoFence` instala la versión portátil y evita el aviso de SmartScreen.

<details>
<summary><strong>Requisitos, configuración y algunas notas útiles</strong></summary>

- Windows 10 22H2, o Windows 11 22H2 o posterior. La Configuración necesita Microsoft Edge WebView2 Runtime. Windows 11 lo incluye; en Windows 10 instálalo desde Microsoft si la Configuración no se abre.
- El primer inicio crea los grupos «Programas», «Carpetas», «Archivos y documentos» y «Escritorio» en el idioma que elijas. Los iconos del escritorio de Windows se restauran al salir.
- La configuración se guarda en `%APPDATA%\PecoFence\config.json`. Ejecuta la aplicación con `--portable` para guardarla en una carpeta `config` junto al ejecutable.
- El ZIP y el instalador no están firmados. Si Windows SmartScreen aparece en el primer inicio, elige **Más información → Ejecutar de todas formas**.

[Guía de la edición portátil](../PORTABLE.md) · [Guía de idiomas](../LOCALIZATION.md) · [Guía del instalador de Windows](../INSTALLER.md)

</details>

## Constrúyelo. Hazlo tuyo.

PecoFence tiene licencia Apache 2.0 y las contribuciones son bienvenidas: desde una traducción más
precisa hasta una interacción de escritorio mejor resuelta.

[Contribuir](../../CONTRIBUTING.md) · [Mejorar una traducción](../LOCALIZATION.md) · [Guía de desarrollo](../DEVELOPMENT.md)

---

**Hecho para un escritorio al que da gusto volver.**  
[Licencia Apache 2.0](../../LICENSE) · [Avisos de terceros](../../third_party/README.md)

Gracias a la comunidad [LINUX DO](https://linux.do).
