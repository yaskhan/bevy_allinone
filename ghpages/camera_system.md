# Camera system

## Обзор

Camera system — это набор компонентов и систем, отвечающих за управление камерой в 3D/2.5D проектах. Он поддерживает несколько режимов (Third Person, First Person, Locked, Side Scroller, Top Down), плавное следование за персонажем, переключение плеча, динамическое приближение, ограничения по углам, коллизии с окружением, прицеливание, пост-эффекты (shake, bob), а также сценарные зоны камеры и патрулирование по путевым точкам.

Система организована вокруг компонента `CameraController`, который хранит настройки поведения, и `CameraState`, который содержит текущие вычисленные значения (yaw/pitch, дистанция, смещение и т.д.). Дополнительные подсистемы реализованы отдельными модулями (`follow`, `collision`, `fov`, `shake`, `bob`, `lock`, `zones`, `state_offsets`, `collision_lean`).

## Основные сущности и компоненты

### CameraController
`CameraController` — ключевой компонент управления поведением камеры.

Основные поля:

- `follow_target: Option<Entity>` — цель, за которой следует камера (обычно игрок).
- `mode: CameraMode` — активный режим камеры.
- `current_side: CameraSide` — выбранная сторона при плече (Left/Right).
- Чувствительность вращения:
  - `rot_sensitivity_3p` — для Third Person.
  - `rot_sensitivity_1p` — для First Person.
  - `aim_zoom_sensitivity_mult` — множитель чувствительности при прицеливании.
- Ограничения углов:
  - `min_vertical_angle`, `max_vertical_angle` — лимиты pitch.
- Дистанция:
  - `distance`, `min_distance`, `max_distance` — текущая и предельные дистанции.
- Сглаживание:
  - `smooth_follow_speed` — общий параметр скорости слежения.
  - `smooth_rotation_speed` — скорость сглаживания вращения.
  - `pivot_smooth_speed` — сглаживание точки поворота.
  - `distance_smooth_speed` — сглаживание изменения дистанции.
- Смещения:
  - `side_offset` — боковой оффсет камеры относительно цели.
  - `default_pivot_offset` — базовое смещение pivot.
  - `aim_pivot_offset` — смещение при прицеливании.
  - `crouch_pivot_offset` — смещение при приседе.
- Lean (наклон/выглядывание):
  - `lean_amount`, `lean_angle`, `lean_speed`, `lean_raycast_dist`, `lean_wall_angle`.
- FOV:
  - `default_fov`, `aim_fov`, `fov_speed`.
- Коллизии:
  - `use_collision`, `collision_radius`.
- Target Lock:
  - `target_lock: TargetLockSettings` (подробности ниже).
- Базовые настройки для плавного возврата после зоны:
  - `base_mode`, `base_distance`, `base_fov`, `base_pivot_offset`, `base_transition_speed`.

### CameraState
`CameraState` хранит динамические величины, которые пересчитываются каждый кадр:

- `yaw`, `pitch` — углы ориентации.
- `current_distance` — текущая дистанция камеры с учетом сглаживания.
- `current_pivot` — вычисленная точка поворота (eye position).
- `current_side_interpolator` — интерполируемое значение стороны (-1..1).
- `current_lean` — текущее значение наклона.
- `noise_offset` — шум/дрожание для эффекта дыхания и shake.
- `bob_offset` — текущий сдвиг из bobbing эффекта.
- `is_aiming`, `is_crouching` — состояние флага прицеливания/приседа.
- `fov_override`, `fov_override_speed` — опциональное переопределение FOV.

### CameraMode
`CameraMode` задает режим поведения:

- `ThirdPerson` — камера следует за персонажем.
- `FirstPerson` — камера фиксируется в позиции глаз.
- `Locked` — отключение ручного вращения.
- `SideScroller` — боковой режим.
- `TopDown` — вид сверху.

### CameraTargetState
Компонент управления прицеливанием и фиксацией цели:

- `marked_target` — цель, подсвеченная для потенциального lock-on.
- `locked_target` — активная цель для lock-on.
- `is_locking` — признак режима фиксации.

### CameraZone / CameraZoneTracker
`CameraZone` описывает зону, которая может менять режим и настройки камеры. Поля:

- `settings: CameraZoneSettings` — целевые параметры.
- `priority` — приоритет зоны.

`CameraZoneTracker` прикрепляется к игроку и хранит список активных зон и текущую выбранную зону.

### CameraWaypoint / CameraWaypointTrack / CameraWaypointFollower
Эти компоненты используются для сценарных камер:

- `CameraWaypoint` — узел маршрута с настройкой ожидания, скорости и ориентации.
- `CameraWaypointTrack` — список узлов и опциональный loop.
- `CameraWaypointFollower` — текущий индекс и состояние следования.

### CameraBobState и BobPreset
`CameraBobState` содержит набор пресетов (idle, walk, sprint, aim) и текущие смещения (позиционные/вращательные) для эффекта качания камеры. `BobPreset` определяет амплитуды, скорости и параметры сглаживания.

### TargetLockSettings
Настройки механики захвата цели:

- `enabled` — включение режима lock-on.
- `max_distance` — максимальная дистанция до цели.
- `fov_threshold` — допустимый угол отклонения цели от центра.
- `scan_radius` — зона «липкости» вокруг центра экрана.
- `lock_smooth_speed` — скорость поворота к цели.

## Подсистемы и ключевые процессы

### 1. Вращение и управление камерой
Система `update_camera_rotation` обрабатывает ручное вращение камеры по вводу `InputState`:

- Чувствительность зависит от режима (`rot_sensitivity_3p` / `rot_sensitivity_1p`).
- При прицеливании применяется множитель `aim_zoom_sensitivity_mult`.
- Ограничение pitch между `min_vertical_angle` и `max_vertical_angle`.
- В режиме `Locked` ручное вращение отключено.

### 2. Смещения и pivot (`state_offsets`)
Система `update_camera_state_offsets`:

- Синхронизирует позицию pivot с трансформом цели.
- Обрабатывает смену плеча (`CameraSide`) по `InputState`.
- Учитывает присед, прицеливание и состояние движения через `CharacterMovementState`.
- Плавно интерполирует положение pivot.

### 3. Следование (follow)
Система `update_camera_follow`:

- Использует `current_pivot` как точку отсчета.
- Добавляет lean-смещение и bobbing.
- Рассчитывает конечный поворот по yaw/pitch + шум.
- Сглаживает вращение и дистанцию.

### 4. Коллизии (collision)
`handle_camera_collision` использует `SpatialQuery` из avian3d:

- Выполняет raycast от pivot к позиции камеры.
- Если обнаружено препятствие, камера смещается ближе к pivot, чтобы избежать пересечения.

### 5. FOV (fov)
`update_camera_fov`:

- Приоритеты: `fov_override` > `aim_fov` > `default_fov`.
- Поддерживает плавное изменение с `fov_speed`.

### 6. Shake (shake)
`ShakeQueue` — ресурс, позволяющий добавлять shake-запросы из любых систем.

- `update_camera_shake` создает `CameraShakeInstance`.
- Каждая инстанция рассчитывает смещения на основе синусоидального шума.
- Итоговые оффсеты добавляются к `noise_offset`.

### 7. Bobbing (bob)
`update_camera_bob`:

- Выбирает пресет в зависимости от движения игрока и прицеливания.
- Рассчитывает позиционные/вращательные колебания.
- Записывает `bob_offset` и корректирует `noise_offset` для легкого качания.

### 8. Lean (collision_lean)
`update_camera_lean_collision`:

- Слушает `InputState` (`lean_left`, `lean_right`).
- Проверяет коллизии через raycast (чтобы не застревать в стене).
- Плавно интерполирует `current_lean`.

### 9. Target Lock (lock)
Система `update_target_marking` ищет лучшую цель:

- Учитывает угол к forward-вектору, расстояние и здоровье.
- Вычисляет score, выбирает наиболее подходящую цель.

`update_target_lock`:

- Триггерится по `InputState.lock_on_pressed`.
- Плавно вращает yaw/pitch к цели (`lock_smooth_speed`).
- Сбрасывает lock при выходе из дистанции.

### 10. Camera Zones (zones)
`update_camera_zones` и `apply_camera_zone_settings`:

- Определяют, какие зоны активны, используя `SpatialQuery` и позицию игрока.
- Выбирают зону с максимальным приоритетом.
- Плавно применяют настройки зоны (mode, distance, fov, pivot).
- При выходе из зоны возвращают базовые настройки.

## Создание камеры

Рекомендуемый способ — использовать helper `spawn_camera`:

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn setup(mut commands: Commands) {
    let player_entity = commands.spawn((Player, CharacterController::default())).id();
    spawn_camera(&mut commands, player_entity);
}
```

`spawn_camera` автоматически:

- Создает `Camera3d`.
- Добавляет `CameraController`, `CameraState`, `CameraBobState`, `CameraTargetState`.
- Инициализирует позицию и поворот.

## Пример настройки

```rust
fn configure_camera(mut query: Query<&mut CameraController>) {
    if let Ok(mut camera) = query.get_single_mut() {
        camera.distance = 6.0;
        camera.min_distance = 2.0;
        camera.max_distance = 12.0;
        camera.default_fov = 70.0;
        camera.aim_fov = 45.0;
        camera.use_collision = true;
        camera.target_lock.enabled = true;
    }
}
```

## Интеграция с вводом

Camera system использует `InputState` из модуля `input`, поэтому:

- `InputState.look` — управление yaw/pitch.
- `InputState.aim_pressed` — влияет на FOV и pivot.
- `InputState.switch_camera_mode_pressed` — переключение режимов.
- `InputState.side_switch_pressed` — смена плеча.
- `InputState.lean_left` / `InputState.lean_right` — выглядывание.
- `InputState.lock_on_pressed` — захват цели.

## Подключение плагина

`CameraPlugin` автоматически включается в `GameControllerPlugin`, но его можно добавить отдельно:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CameraPlugin)
    .run();
```

## Рекомендации по расширению

- Для сценарных камер используйте `CameraWaypointTrack` + `CameraWaypointFollower`.
- Для изменения поведения камеры на локациях создайте сущности с `CameraZone` и коллайдерами.
- Для сильных кинематографических эффектов используйте `ShakeQueue`.
- Для модов или дополнительных режимов можно расширить `CameraMode` и добавить новые условия в `update_camera_follow` и `update_camera_state_offsets`.
