---
id: 33
title: Debug Console avec commandes
status: Done
priority: medium
milestone: MVP5-Debug
assignees:
  - '@claude'
labels:
  - phase5
  - debug
  - console
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-01-18T20:10:24.955Z'
updated_date: '2026-01-19T05:22:50.601Z'
closed_date: '2026-01-19T05:22:50.601Z'
changelog:
  - timestamp: '2026-01-18T20:10:24.955Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-01-19T05:17:24.509Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-01-19T05:17:51.791Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:22:36.752Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-01-19T05:22:50.601Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Ameliorer la debug console avec plus de commandes (teleport, pos, speed, etc.)

  ### Etapes
  1. Creer enum ConsoleCommand pour les commandes parsees
  2. Ajouter queue de commandes pending dans DebugOverlay
  3. Refactorer execute_command pour parser les arguments
  4. Ajouter methode take_pending_commands() pour le jeu
  5. Implementer nouvelles commandes:
     - pos: afficher position joueur
     - tp/teleport x y: teleporter joueur
     - speed <value>: changer vitesse
     - zorder on/off: toggle z-order
     - timescale <value>: changer vitesse du jeu
     - entities: lister entites ECS
     - stats: afficher render stats
  6. Integrer dans game/main.rs pour executer les commandes

  ### Fichiers concernes
  - crates/engine_debug/src/overlay.rs (ConsoleCommand, parsing, pending queue)
  - crates/engine_debug/src/lib.rs (exports)
  - game/src/main.rs (execution des commandes)

  ### Approche technique
  - ConsoleCommand enum avec variants pour chaque commande + args
  - Parsing avec split_whitespace et parse::<f32>()
  - Vec<ConsoleCommand> pour les commandes pending
  - Le jeu poll les commandes chaque frame

  ### Defis potentiels
  - Acces au World depuis la console (resolu via pending commands)
  - Parsing robuste des arguments
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] ConsoleCommand enum created with all command variants

  - [x] Pending commands queue in DebugOverlay

  - [x] Command parsing with arguments support

  - [x] Console history with up/down navigation

  - [x] All commands implemented: help, clear, fps, stats, pos, tp, speed,
  timescale, entities, collision, zorder

  - [x] Game-side command processing integrated


  ### Tests effectues

  - Build avec debug-tools: OK

  - Game startup: OK


  ### Qualite du code

  - Standards respectes: Oui

  - Documentation: Partielle (inline comments)

  - Error handling: Oui (invalid arguments handled)


  ### Commandes disponibles

  | Commande | Description |

  |----------|-------------|

  | help | Liste des commandes |

  | clear | Efface la console |

  | fps | Affiche FPS/UPS |

  | stats | Affiche stats de rendu |

  | pos | Position du joueur |

  | tp x y | Teleporte le joueur |

  | speed n | Change vitesse joueur |

  | timescale n | Change echelle temps |

  | entities | Liste les entites |

  | collision | Toggle collision debug |

  | zorder | Toggle z-order debug |


  ### Limitations connues

  - Pas de completion automatique

  - Pas de sauvegarde historique entre sessions


  ### Recommandations

  - Ajouter auto-completion plus tard

  - Ajouter commandes spawn/give quand l'inventaire sera implementé
---
Console in-game avec commandes (teleport, spawn, give item, etc) (F7)
