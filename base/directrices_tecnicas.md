# Directrices Técnicas - Proyecto Financiero Senior
Este documento sirve como guía arquitectónica y técnica para el desarrollo del sistema financiero, consolidando los estándares, patrones y decisiones de diseño discutidos.

## 1. Visión Arquitectónica General
El objetivo es construir un sistema de **alta capacidad**, **tolerante a fallos** y con **cero tiempo de inactividad** (Zero Downtime).

### Estilos de Arquitectura
*   **Modular Monolith / Microservicios**: Desacoplamiento de dominios con aislamiento de fallos (Bulkheads).
*   **Event-Driven Architecture (EDA)**: Comunicación asíncrona para escritura y consistencia eventual controlada.
*   **Hexagonal / Clean Architecture**: Independencia total del framework y la base de datos en el núcleo de negocio. Core funcional puro (especialmente en Rust).

## 2. Estándares de Seguridad (Critical Path)
*   **Modelo Zero Trust**: "Nunca confiar, siempre verificar". Micro-segmentación.
*   **OWASP ASVS**: Cumplimiento de Niveles 2 o 3.
*   **Autenticación y Autorización**:
    *   **OAuth 2.1 / OIDC**: Gestión de identidad federada.
    *   **RBAC / ABAC**: Control de acceso granular.
    *   **MFA**: FIDO2/WebAuthn mandatorio. Evitar SMS.
    *   **Tokens**: PASETO o JWT cifrados. Short-lived access tokens + Rotación de Refresh Tokens.
    *   **Almacenamiento**: Cookies `HttpOnly`, `Secure`, `SameSite=Strict` (BFF Pattern). Prohibido localStorage para tokens.
*   **Cifrado**: TLS 1.3 en tránsito. AES-256 en reposo. Gestión de secretos vía Vault/HSM.

## 3. Stack Tecnológico Estrictamente Definido
*   **Frontend**: Next.js + React (TypeScript estricto) + MUI.
    *   **Patrón**: Backend for Frontend (BFF) en Next.js para agregación y manejo de sesión.
*   **Backend Core**: Rust (Axum).
    *   **Enfoque**: High Performance, Memory Safe.
    *   **Patrón**: Functional Core, Imperative Shell.
*   **Base de Datos**: PostgreSQL.
    *   **Modelo**: Relacional estricto + JSONB para metadatos.
*   **Event Broker**: Kafka / Redpanda.
    *   **Rol**: Backbone de eventos duraderos, orden garantizado, source of truth secundario.
*   **Caché/Coordinación**: Redis.
    *   **Uso estricto**: Idempotency keys, colas simples, caché de lectura (read model). No persistencia crítica.

## 4. Diseño del Core Financiero (Ledger)
### Principios del Ledger
*   **Doble Entrada**: Todo movimiento tiene debe y haber. Suma cero por transacción obligatoria.
*   **Inmutabilidad (Append-Only)**: `UPDATE` prohibido en saldos. El saldo es una vista derivada (`sum(postings)`).
*   **Tipos de Datos**: Enteros (`BigInt`) para montos. Nunca `Float`.

### Esquema de Base de Datos (PostgreSQL)
*   **Aislamiento**: `SERIALIZABLE` para escrituras en Ledger. Alternativa: `READ COMMITTED` + `SELECT ... FOR UPDATE`.
*   **Integridad**: Constraints o Triggers para asegurar suma cero en transacciones.

### Idempotencia
Mecanismo obligatorio para evitar duplicidad de cargos.
1.  Cliente envía `Idempotency-Key` (UUID).
2.  Middleware Rust verifica en Redis/DB.
3.  Si existe y terminó -> Retorna respuesta guardada.
4.  Si no existe -> Procesa atómicamente.

## 5. Patrones de Programación y Diseño
### Backend (Rust)
*   **Evolución de SOLID -> CUPID / Data-Oriented Design (DOD)**:
    *   Priorizar composición sobre herencia.
    *   Diseño orientado a la memoria y datos para performance.
*   **Manejo de Errores**: Tipados y exhaustivos. No `panics` en producción.
*   **Validación**: DTOs validados al entrar (librería `validator`).

### Frontend (React/Next.js)
*   **State Management**:
    *   **Server State**: TanStack Query (React Query). Caching, deduplicación.
    *   **Client/UI State**: Zustand.
*   **Lógica de UI**: Máquinas de Estado Finito (XState) para flujos complejos (evitar boolean flags inmanejables).
*   **Optimistic UI**: Actualización inmediata con rollback automático on-error.
*   **Componentes**: Compound Components para UI compleja. Evitar HOCs, preferir Hooks.

## 6. Comunicación y API
*   **Modelo Híbrido**:
    *   **REST (o gRPC interno)**: Para Comandos y Escrituras (Atomicidad).
    *   **WebSockets (SSE)**: Para Eventos y notificaciones Real-time (Saldos actualizados).
    *   **NO GraphQL en el Core**: Solo permitido en capa BFF (Next.js) si es estrictamente necesario para UI flexible.
*   **Documentación**: OpenAPI (Swagger) Code-First generado desde Rust.

## 7. Ingeniería, Calidad y Operaciones
### Observabilidad (Day 2 Ops)
*   **Distributed Tracing**: OpenTelemetry (Trace ID único end-to-end).
*   **Logs**: Estructurados (JSON), sin PII.
*   **Métricas**: SLIs/SLOs de negocio (latencia de ledger, tasa de fallos).

### Testing Avanzado
*   **Property-Based Testing**: (`proptest` en Rust) para hallar bugs matemáticos/lógicos complejos.
*   **Chaos Engineering**: Inyección de fallos controlada.
*   **Load Testing**: Validación de throughput.

### Infraestructura (IaC)
*   **Despliegue**: Inmutable (Contenedores/K8s).
*   **GitOps**: ArgoCD/Flux.
*   **Multi-Región**: Activo-Activo con failover automático para alta disponibilidad.
