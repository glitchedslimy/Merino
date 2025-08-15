# TODO List

### Backend

#### Notas
- [ ] Implementar el método "read_note_file"
- [ ] Implementar el método "load_note_content"
- [ ] Implementar el método "update_note_content"
- [ ] Implementar el método "delete_note"
- [ ] Implementar el método "rename_note"
- [ ] Implementar la funcionalidad de "Notas Rápidas"

---

#### Espacios
- [ ] Implementar el método "create_space"
- [ ] Implementar el método "delete_space"
- [ ] Hacer posible el uso de diferentes "Espacios" en diferentes rutas del sistema, no solo la predeterminada

---

#### IA
- [ ] Implementar todas las funcionalidades de "IA" de Ollama y tratar de modificarlas para incluir también modelos externos
- [ ] Implementar el acceso a Internet para modelos locales (Ollama)
- [ ] Ver cómo puedo implementar el sidecar para usar MCPs (Multi-modal Command Processing) con la IA
- [ ] Implementar un mejor mecanismo de cancelación para el chat de IA

---

#### Misceláneo
- [ ] Optimizar el backend siguiendo el patrón hexagonal + DDD + **Featured based**
- [ ] Implementar la funcionalidad de ajustes guardando el archivo en formato JSON o YAML
- [ ] Ver cómo poner colores en los logs y hacer un mejor uso del registro y manejo de errores
- [ ] Hacer un sistema de plugins y una tienda para ello
- [ ] Hacer posible que el usuario sincronice las notas entre dispositivos (a futuro)
- [ ] Hacer posible la creación de carpetas para organizar las notas

***

### Frontend

- [ ] Refactorizar todos los componentes y hacer un mejor uso de CVA y la componetización
- [ ] Arreglar y modificar las funciones para que las llamadas entre FE y BE funcionen correctamente
- [ ] Encontrar una arquitectura adecuada para el frontend para este caso de uso
- [ ] Optimizar el frontend
- [ ] Añadir el panel de ajustes
- [ ] Implementar la funcionalidad de "búsqueda web"
- [ ] Implementar el panel de comandos
- [ ] Implementar el panel de búsqueda
- [ ] Crear la tienda de plugins para tener plugins **plug-and-play**
- [ ] Crear un sistema de notificaciones en lugar de solo toasts. Si un toast aparece y el usuario no lo cierra, hacer un contenedor de notificaciones para que se queden ahí
- [ ] Implementar plantillas
- [ ] Implementar modo de solo lectura
- [ ] Implementar más funcionalidades del editor y **Markdown**
- [ ] Hacer un mecanismo de arrastrar y soltar para ordenar las notas en carpetas y lo que sea

***

# IDEAS List

### Ideas de IA Adicionales
- Generación de resúmenes de notas largos.
- Funcionalidad para responder preguntas sobre el contenido de las notas.
- Generación de ideas para nuevas notas basadas en el contenido existente.
- Análisis de sentimientos sobre el contenido de las notas para ayudar a organizar los pensamientos.

### Funcionalidades de Contenido y Editor
- Kanban Board View para visualizar las notas como tarjetas.
- Mind Mapping para la organización visual de ideas.
- Inserción de contenido enriquecido como videos o PDFs directamente en las notas e implementaciones de terceros como webs (iFrames), Github...

### Mejoras de IA
- Etiquetado y categorización automatizados de notas.
- Conexiones de notas basadas en el contenido para crear un grafo de conocimiento personal.
- Funcionalidad de IA para refactorizar o reescribir notas.

### Experiencia de Usuario y Flujo de Trabajo
- Historial de versiones para cada nota.
- Sistema de revisión diaria/repetición espaciada para notas antiguas.
- Web Clipper Extension
- Notes sync
- Exposed API for developers
- Tag Bundles
- Visual Content Library
- Notebooks and collections