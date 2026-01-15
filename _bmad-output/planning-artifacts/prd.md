---
stepsCompleted: ['step-01-init', 'step-02-discovery', 'step-03-success', 'step-04-journeys', 'step-06-innovation', 'step-07-project-type', 'step-08-scoping', 'step-09-functional', 'step-10-nonfunctional', 'step-11-complete']
inputDocuments:
  - '_bmad-output/analysis/brainstorming-session-2026-01-09.md'
workflowType: 'prd'
lastStep: 11
workflowComplete: true
completedDate: '2026-01-10'
documentCounts:
  briefCount: 0
  researchCount: 0
  brainstormingCount: 1
  projectDocsCount: 0
---

# Product Requirements Document - vocal-note-taker

**Author:** Tamles
**Date:** 2026-01-09

## Executive Summary

**vocal-note-taker** est une application desktop de capture vocale ultra-rapide conçue pour transformer des prompts vocaux courts (30 secondes à 2 minutes) en texte exploitable. Optimisée pour un usage personnel sur Ubuntu et Mac, l'application se concentre sur un workflow linéaire sans friction : enregistrer → transcrire localement → copier → utiliser ailleurs.

**Vision du Produit :**
Créer un outil "invisible" qui disparaît dans le workflow quotidien - un pont temporaire entre la voix et le texte, pas un gestionnaire de notes. L'application reste en arrière-plan (ghost mode) et réagit instantanément aux raccourcis clavier, permettant de capturer rapidement des idées complexes sans interrompre le flux de travail.

**Cas d'Usage Principaux :**
- Capture de prompts pour ChatGPT, Claude et autres IA conversationnelles
- Rédaction rapide de messages Teams ou autres communications
- Dictée courte dans tout contexte nécessitant de la vitesse vs typing manuel

**Problème Résolu :**
Éliminer la friction entre "j'ai une idée complexe" et "elle est capturée et prête à être utilisée". Les solutions existantes sont soit trop lentes (typing manuel), soit compromettent la vie privée (transcription cloud), soit conçues pour de la dictée longue avec gestion d'historique complexe.

**Architecture Technique :**
- **Framework :** Tauri (Rust + HTML/CSS/JS) pour interface minimale cross-platform
- **Backend :** Rust pour capture audio et orchestration
- **Transcription :** whisper.cpp local (modèle large) via whisper-rs bindings pour processing rapide et privé
- **Plateformes :** Ubuntu (priorité), Mac (phase 2)

### What Makes This Special

**1. Privacy-First Architecture**
La data vocale ne quitte jamais la machine locale. Toute la transcription se fait via whisper.cpp en local, sans dépendance cloud ni envoi de données. Dans un monde où les solutions dominantes (Google, Apple, Azure Speech) envoient tout au cloud, vocal-note-taker garantit que vos prompts privés, idées sensibles et communications restent sur votre poste.

**2. Quick Capture Optimisé**
Design spécialisé pour captures ultra-courtes (30sec-2min) vs dictée générique longue. Cette spécialisation permet des optimisations radicales :
- Effacement automatique au nouvel enregistrement (workflow linéaire)
- Pas d'historique ni de gestion de fichiers
- Copie clipboard ultra-rapide (un clic ou Enter)
- Interface minimale sans distractions

**3. Ghost Mode Push-to-Talk**
L'application n'existe presque pas visuellement jusqu'à nécessité. Reste en arrière-plan invisible et réagit instantanément au raccourci clavier global. Pas de changement de contexte visuel, pas d'ouverture de fenêtre intrusive - workflow ultra-fluide.

**4. Contrôle Manuel & Respect Environnement**
Refuse de prendre des décisions automatiques affectant le système. Pas de copie automatique dans le clipboard - l'utilisateur décide QUAND copier. Respect total de l'environnement système et du workflow existant.

**5. Power User First**
Conçu pour utilisateurs avancés privilégiant efficacité et contrôle :
- Raccourcis clavier entièrement configurables
- Keyboard-first (auto-focus + Enter pour copier)
- Feedback visuel continu (sound wave, timer, indicateurs)
- Transparence technique totale

**Le Moment "Aha" :**
Lancer la transcription via raccourci global, voir le texte apparaître en quelques secondes, appuyer sur Enter pour copier - le tout sans quitter l'application en cours. Gain de vitesse massif vs typing manuel, avec garantie privacy totale.

**Hypothèse Challengée :**
Les applications de dictée vocale ne doivent PAS envoyer vos données au cloud pour être performantes. Le processing local moderne (whisper.cpp) est suffisamment rapide et précis pour un workflow professionnel, tout en préservant privacy et autonomie.

## Project Classification

**Technical Type:** Desktop App
**Domain:** General (Personal Productivity)
**Complexity:** Low
**Project Context:** Greenfield - new project

**Justification de la Classification :**

**Desktop App :**
- Application native cross-platform via Tauri
- Exécution locale avec interface système (raccourcis globaux)
- Modes ghost/background persistants
- Intégration clipboard système
- Pas de serveur distant ni dépendances web

**Domain General :**
Outil de productivité personnelle sans contraintes réglementaires spécifiques. Pas de compliance healthcare, fintech, ou autre domaine réglementé. Usage personnel donc pas de multi-tenant, RBAC complexe, ou considérations enterprise.

**Complexity Low :**
Bien que l'intégration technique (Tauri + Rust + whisper.cpp) nécessite des compétences, le domaine fonctionnel reste simple : capture audio → transcription → copie. Pas de logique métier complexe, pas de workflows multi-étapes, pas de règles réglementaires. La complexité est technique (intégration composants) mais pas fonctionnelle. Architecture unifiée en Rust simplifie l'intégration vs multi-langage.

**Implications pour le PRD :**
- Focus sur UX/UI desktop patterns et interactions clavier
- Sections critiques : platform requirements, system integration, offline capabilities
- Sections non-applicables : web SEO, mobile features, multi-tenant, compliance
- Priorité : performance locale, intégration OS, raccourcis système

## Success Criteria

### User Success

**vocal-note-taker** réussit pour l'utilisateur quand il offre un workflow de capture vocale qui devient naturel et sans friction dans le quotidien.

**Critères de Succès Utilisateur :**

1. **Vitesse de Capture Totale < 15 secondes**
   - De l'appui sur le raccourci clavier à la copie dans le clipboard : moins de 15 secondes
   - Workflow complet : raccourci → parler → transcription → scan visuel → copier
   - Aucune interruption du contexte de travail actuel

2. **Qualité de Transcription ≥ 90%**
   - Exactitude minimale de 90% des mots correctement transcrits
   - Priorité absolue : qualité > vitesse
   - Acceptable de prendre plus de temps si la qualité est au rendez-vous
   - Correction mineure acceptable (quelques mots), réécriture complète inacceptable

3. **Confiance Technique Continue**
   - Zéro moment de doute pendant l'enregistrement
   - Feedback visuel continu (sound wave, timer, indicateur REC)
   - L'utilisateur sait toujours si le micro capte correctement
   - Transparence totale sur l'état du système

4. **Contrôle Total du Workflow**
   - Décision manuelle sur QUAND copier dans le clipboard
   - Raccourcis clavier pour toutes les actions critiques
   - Keyboard-first : Enter pour copier sans toucher la souris
   - Aucune action surprise affectant l'environnement système

5. **Tolérance aux Échecs Réaliste**
   - Quelques échecs occasionnels acceptables (environnement trop bruyant, micro non détecté)
   - Messages d'erreur clairs et actionnables
   - Récupération gracieuse sans crash ni perte de données
   - Pas de tolérance zéro - fiabilité raisonnable suffit

**Le Moment "Ça Marche" :**
L'utilisateur lance la transcription via raccourci global, parle naturellement, voit le texte apparaître avec 90%+ d'exactitude, appuie sur Enter pour copier - le tout sans quitter l'application en cours. Ce workflow devient réflexe après 1 semaine d'utilisation.

### Business Success

Pour un projet personnel, le succès business se mesure à l'adoption réelle et au remplacement durable de comportements existants.

**Critères de Succès Business :**

1. **Timeline de Validation : 1 Semaine d'Usage**
   - Après 1 semaine d'utilisation quotidienne réelle, l'app doit être jugée "indispensable"
   - Validation que le workflow répond au besoin initial
   - Décision go/no-go sur investissement phase 2

2. **Fréquence d'Usage : 5-10x par Jour**
   - Usage quotidien minimum : 5 captures vocales
   - Usage optimal : 10+ captures par jour
   - Signe d'adoption réussie : l'app devient le réflexe par défaut

3. **Remplacement du Typing Manuel**
   - Substitution mesurable : prompts IA tapés manuellement → dictés vocalement
   - Extension naturelle : messages Teams, emails courts également dictés
   - Gain de temps ressenti et maintenu sur plusieurs semaines

4. **Investissement Temps Développement : 2 Semaines Max**
   - MVP complet et utilisable quotidiennement livré en 2 semaines
   - Si dépassement significatif (>3 semaines), réévaluation du ROI
   - Développement assisté par IA pour respecter timeline

5. **Maintenance Légère : Quelques Heures/Mois**
   - Après MVP, maintenance minimale requise
   - Stabilité suffisante pour ne pas nécessiter interventions constantes
   - Budget temps : 2-4h/mois max pour bug fixes et petites améliorations

**Indicateur de Réussite Globale :**
Après 1 mois d'utilisation, l'utilisateur ne peut plus imaginer revenir au typing manuel pour les prompts IA. L'app est ouverte en permanence en arrière-plan et utilisée sans y penser.

### Technical Success

Le succès technique garantit que l'architecture et l'implémentation supportent durablement l'expérience utilisateur cible.

**Critères de Succès Techniques :**

1. **Qualité de Transcription (Whisper.cpp)**
   - Modèle whisper.cpp (tiny ou base) atteignant 90%+ d'exactitude
   - Configuration optimale identifiée pour environnement utilisateur
   - Fallback vers modèle plus gros (base → small) si tiny insuffisant

2. **Réactivité UI Instantanée**
   - Interface réagit immédiatement aux interactions clavier/souris
   - Aucun freeze ou lag perceptible pendant l'enregistrement
   - Communication Tauri ↔ Rust backend non-bloquante (async)

3. **Stabilité Système**
   - **Zéro crash fréquent** - crashes occasionnels acceptables, mais pas répétitifs
   - **Zéro conflit avec autres apps** - cohabitation pacifique avec le reste du système
   - Gestion gracieuse des erreurs (micro occupé, whisper.cpp fail, etc.)
   - Récupération automatique après erreur sans intervention manuelle

4. **Performance Locale Acceptable**
   - Transcription locale fonctionnelle sur hardware cible (Ubuntu desktop standard)
   - Consommation RAM/CPU raisonnable (pas de spike qui ralentit le système)
   - Pas de limite stricte sur taille package - simplicité d'installation prioritaire

5. **Packaging et Distribution**
   - Package .deb installable proprement sur Ubuntu
   - Dépendances clairement documentées (Rust toolchain, whisper-rs, cpal)
   - Installation en <5 minutes sans expertise technique avancée

6. **Maintenabilité Long-Terme**
   - Code suffisamment clair pour modifications futures
   - Architecture modulaire (Tauri frontend / Rust backend / whisper-rs séparés)
   - Documentation minimale pour reprise après pause longue

**Architecture de Succès :**
- Frontend Tauri + Backend Rust + whisper-rs communiquent de manière fiable
- Workflow end-to-end fonctionne à chaque fois (modulo erreurs environnementales)
- L'app peut tourner en arrière-plan pendant des jours sans problème
- Architecture unifiée Rust élimine les problèmes IPC inter-langage

### Measurable Outcomes

**Métriques Quantitatives :**

| Métrique | Cible MVP | Méthode de Mesure |
|----------|-----------|-------------------|
| Temps workflow complet | < 15 secondes | Timer manuel du raccourci à la copie |
| Exactitude transcription | ≥ 90% | Comparaison texte transcrit vs texte attendu sur 10 samples |
| Fréquence d'usage quotidien | 5-10x/jour | Log interne ou observation comportementale |
| Délai de validation | 1 semaine | Date première utilisation → décision "indispensable" |
| Timeline développement | ≤ 2 semaines | Date début dev → date MVP utilisable |
| Taux de crashes | < 1 par semaine | Observation sur 1 mois d'usage |
| Temps maintenance mensuel | < 4 heures | Suivi temps investi post-MVP |

**Métriques Qualitatives :**

- **"Je ne peux plus m'en passer"** - Ressenti subjectif après 1 mois
- **"C'est devenu un réflexe"** - Automatisme dans le workflow quotidien
- **"Je ne doute jamais"** - Confiance totale pendant l'enregistrement
- **"Ça disparaît dans mon workflow"** - Outil invisible jusqu'à nécessaire

**Success Validation Timeline :**
- **Jour 14 :** MVP livré et installé
- **Jour 21 :** 1 semaine d'usage réel, validation go/no-go
- **Jour 45 :** 1 mois d'usage, décision sur phase 2

## Product Scope

### MVP - Minimum Viable Product (2 Semaines)

**Must-Have - Sans ça, l'app n'est pas utilisable :**

1. **Enregistrement Audio Fonctionnel**
   - Bouton start/stop basique (ou raccourci clavier simple)
   - Capture audio via cpal (Rust)
   - Sauvegarde WAV temporaire 16kHz mono

2. **Transcription Locale Whisper.cpp**
   - Intégration whisper.cpp via whisper-rs bindings
   - Modèle large (~3GB) pour qualité maximale
   - Pas de fallback - qualité prioritaire

3. **UI Minimale Desktop**
   - Interface Tauri (HTML/CSS/JS)
   - Texte transcrit affiché complet et lisible
   - Bouton "Copier" vers clipboard
   - Bouton "Nouvel enregistrement"

4. **Workflow Linéaire Simple**
   - Timer visible pendant enregistrement
   - Indicateur "🔴 REC" visuel
   - Effacement automatique au nouvel enregistrement
   - Pas d'historique, pas de gestion de fichiers

5. **Copie Clipboard Optimisée**
   - Bouton copier reçoit auto-focus après transcription
   - Raccourci Enter pour copier rapidement
   - Feedback visuel "✓ Copié !"
   - Contrôle manuel (pas de copie automatique)

6. **Feedback Visuel Basique**
   - Sound wave temps réel pendant enregistrement
   - Timer affichant durée en secondes
   - État clair (idle / recording / transcribing / ready)

7. **Package Installable Ubuntu**
   - .deb généré via Tauri build
   - Documentation installation dépendances (whisper.cpp)
   - Testé sur Ubuntu 22.04+

**Quick Wins Inclus dans MVP :**
- Flat design minimaliste (couleurs douces, police système)
- Fenêtre reste ouverte mais minimisable
- Feedback visuel transitions d'état

**Validation MVP :**
L'utilisateur peut ouvrir l'app, enregistrer un prompt de 30-60 secondes, obtenir la transcription avec 90%+ d'exactitude, copier en un Enter, et coller dans ChatGPT/Claude. Workflow complet en <15 secondes.

### Growth Features (Post-MVP - Semaines 3-4)

**Nice-to-Have - Améliore significativement l'expérience :**

**Phase 2A - Power User Enhancements :**
1. **Push-to-Talk Mode**
   - Alternative au start/stop : maintenir touche = enregistrer
   - Raccourci global configurable
   - Plus intuitif type talkie-walkie

2. **Raccourci Global Show/Hide**
   - Ctrl+Shift+V (ou configurable) pour ramener l'app au premier plan
   - App reste en arrière-plan invisible jusqu'à invocation
   - Ghost mode partiel

3. **Notification Système Post-Transcription**
   - Notification Ubuntu basique "Transcription prête"
   - Clic sur notification ramène app au premier plan
   - Libère l'attention pendant transcription longue

4. **UI Settings Basique**
   - Panel paramètres pour personnalisation
   - Choix modèle whisper (tiny/base/small)
   - Configuration raccourcis clavier
   - Sélection micro si plusieurs disponibles

5. **Indicateur Qualité Audio**
   - Barre niveau dB en temps réel
   - Indicateur pré-enregistrement de qualité environnement
   - Avertissement si trop bruyant

**Phase 2B - Polish & Refinement :**
6. Dark mode (si préférence utilisateur exprimée)
7. Historique optionnel des 5 dernières transcriptions
8. Export texte vers fichier .txt
9. Tests initiaux sur Mac (portage cross-platform)

**Déclencheur Phase 2 :**
Seulement si après 1 mois d'usage MVP, l'utilisateur confirme que l'app est indispensable ET que des frustrations spécifiques émergent que ces features résoudraient.

### Vision (Future - Version 2.0)

**Advanced Features - Vision complète long-terme :**

**Intelligence Audio Avancée :**
1. **Reconnaissance Vocale Personnalisée**
   - Setup initial 5 min pour entraîner à la voix utilisateur
   - Isolation voix principale vs voix environnement
   - Amélioration progressive avec usage

2. **Filtrage Intelligent Autres Voix**
   - Algorithmes ML pour extraire uniquement voix cible
   - Fonctionne en environnement bruyant (café, bureau partagé)
   - Profils environnement prédéfinis

3. **Indicateur Qualité Sophistiqué**
   - 🟢 Optimal / 🟡 Bruyant mais OK / 🔴 Trop bruyant
   - Détection pré-enregistrement pour conseiller timing
   - Apprentissage environnements fréquents

**Ghost Mode Complet :**
4. **Overlay Minimal Pendant Enregistrement**
   - Petit indicateur coin écran (timer + status)
   - App principale reste invisible
   - Push-to-talk global toujours actif

5. **Mode Background Total**
   - App ne s'ouvre jamais en fenêtre complète (optionnel)
   - Notification + overlay suffisent
   - Workflow ultra-fluide sans changement contexte visuel

**Multi-Plateforme & Avancé :**
6. Support Mac optimisé (après validation Ubuntu)
7. Raccourcis entièrement configurables (UI avancée)
8. Intégration directe avec ChatGPT/Claude APIs (paste automatique optionnel)
9. Support multi-langues (français, anglais, autres)

**Déclencheur Version 2.0 :**
Seulement si l'app devient véritablement indispensable pendant 6+ mois ET que des use cases avancés émergent naturellement (environnements bruyants fréquents, besoin Mac urgent, etc.). Vision aspirationnelle, pas roadmap ferme.

## User Journeys

### Journey 1 : Tamles - Le Prompt Complexe à 23h

**Scène d'ouverture :**
Il est 23h, Tamles travaille sur un projet Python complexe. Il a une idée brillante pour résoudre un bug architectural, mais l'explication nécessite du contexte : le pattern actuel, pourquoi il échoue, la solution proposée, et les implications sur 3 autres modules. Taper tout ça prendrait 5-7 minutes et il perdrait le fil de sa pensée à mi-parcours.

**L'action :**
Il appuie sur son raccourci global (Ctrl+Shift+Space). Sans quitter son IDE, un petit overlay apparaît en coin d'écran avec "🔴 REC". Il parle pendant 90 secondes, expliquant toute sa réflexion architecturale de manière fluide et naturelle. Il voit le sound wave qui confirme que le micro capte bien. Il relâche le raccourci.

**Le climax :**
5 secondes plus tard, notification "Transcription prête". Il clique, l'app apparaît au premier plan. Le texte est là, complet, 90%+ correct. Il scanne rapidement (2 secondes), voit que l'essentiel est capturé. Il appuie sur Enter. "✓ Copié !". Il retourne à son IDE, colle dans Claude, et obtient une réponse architecturale détaillée.

**La résolution :**
Total elapsed time : 12 secondes du raccourci à la copie. Tamles a exprimé une pensée complexe en 90 secondes de parole vs 7 minutes de typing, sans perdre le fil, sans quitter son contexte de travail. Le lendemain, cette approche est devenue son réflexe : idée complexe = parler, pas taper.

### Journey 2 : Tamles - Le Message Teams Urgent Pendant une Pause

**Scène d'ouverture :**
Pause café, Tamles consulte Teams sur son téléphone. Un collègue demande un update technique urgent sur l'API qu'il développe. Répondre sur mobile avec clavier tactile = torture. Répondre sur desktop = retourner au bureau. Il veut répondre maintenant, de manière pro, sans frustration.

**L'action :**
Il s'assoit avec son laptop, ouvre vocal-note-taker (déjà en background). Clic "Démarrer". Il parle pendant 45 secondes : "Salut Marc, concernant l'API payment, j'ai finalisé l'endpoint POST /transactions hier. Les tests d'intégration sont à 95%, il reste juste à valider le flow de refund. Je devrais terminer ça demain matin, et on pourra faire la review ensemble en début d'après-midi si tu es dispo."

**Le climax :**
Transcription apparaît. Il scanne : c'est clair, professionnel, complet. Enter. Copié. Il ouvre Teams web, colle le message, envoie.

**La résolution :**
Message professionnel envoyé en 30 secondes total vs 3-4 minutes de typing mobile frustrant ou retour au bureau. Tamles réalise que vocal-note-taker n'est pas juste pour l'IA - c'est pour TOUTE communication textuelle où la vitesse compte plus que la perfection orthographique.

### Journey 3 : Tamles - L'Environnement Bruyant (Edge Case)

**Scène d'ouverture :**
Tamles travaille depuis un café. Deux personnes parlent fort à la table d'à côté. Il veut capturer un prompt pour un projet mais hésite : est-ce que whisper.cpp va transcrire les conversations des autres au lieu de la sienne ?

**L'action :**
Il tente quand même. Raccourci global, parle clairement en dirigeant le micro vers lui. Sound wave est là mais plus erratique (environnement bruyant). Il parle pendant 40 secondes.

**Le climax :**
Transcription apparaît. Il scanne... 70% correct. Whisper a capté quelques mots parasites des conversations environnantes ("project deadline management system" devient "project the deadline no management system"). Pas terrible, mais utilisable avec 10 secondes de correction manuelle.

**La résolution :**
Tamles comprend les limites : environnement calme = 90%+ qualité, environnement bruyant = 70-80% avec corrections. C'est acceptable. Il commence à choisir ses moments pour utiliser l'app, ou trouve des coins plus calmes dans le café. Il sait maintenant que l'app fonctionne même en conditions sous-optimales, juste moins bien. Pas de tolérance zéro = réalisme.

**Future insight :** Cette expérience l'amène à vouloir la feature "indicateur qualité environnement" (🟢🟡🔴) en Phase 2, pour savoir AVANT d'enregistrer si les conditions sont bonnes.

### Journey Requirements Summary

**Ces 3 journeys révèlent les capacités nécessaires suivantes :**

**Capabilities Critiques (MVP) :**
1. **Raccourci global système** - lancer transcription sans quitter app courante
2. **Recording feedback visuel** - sound wave temps réel, timer, indicateur REC
3. **Transcription locale rapide** - whisper.cpp avec qualité 90%+ en environnement calme
4. **UI de review** - affichage texte complet pour scan rapide (2-3 sec)
5. **Copie clipboard optimisée** - auto-focus + Enter pour workflow keyboard-first
6. **Ghost mode / background** - app reste disponible sans être intrusive
7. **Notification post-transcription** - ramène l'attention quand prêt

**Capabilities Phase 2 (Détectées dans edge cases) :**
8. **Indicateur qualité environnement** - prévenir si conditions sous-optimales
9. **Filtrage audio intelligent** - améliorer qualité en environnement bruyant

**Capabilities Long-Terme (Vision) :**
10. **Reconnaissance vocale personnalisée** - isolation voix Tamles vs autres voix

## Innovation & Novel Patterns

### Detected Innovation Areas

**vocal-note-taker** présente plusieurs innovations authentiques qui le différencient des solutions de dictée vocale existantes :

**1. Privacy-First Local AI Processing**

L'application challenge l'hypothèse dominante que la transcription vocale de qualité professionnelle nécessite le cloud. En utilisant whisper.cpp localement, vocal-note-taker démontre que le processing local moderne est suffisamment rapide et précis pour un workflow professionnel.

**Innovation :** Transcription locale avec qualité équivalente aux APIs cloud (Google Speech, Azure, Apple Dictation) qui envoient toutes les données au cloud.

**Validation :** Tests utilisateur avec whisper.cpp (modèle tiny/base) montrent une exactitude ≥90% en environnement calme, suffisant pour le use case cible.

**2. Ghost Mode + Push-to-Talk Workflow**

Réinvente l'UX traditionnelle des apps de dictée qui s'ouvrent en fenêtre complète et interrompent le contexte de travail. L'app reste invisible en arrière-plan et réagit instantanément aux raccourcis globaux.

**Innovation :** L'application "n'existe presque pas visuellement" jusqu'à nécessité - workflow ultra-fluide sans changement de contexte.

**Validation :** Approche testable dans MVP. Si l'UX ghost mode ne convient pas, peut être simplifiée en fenêtre traditionnelle sans impact sur les fonctionnalités core.

**3. Contrôle Manuel vs Automatisation Silencieuse**

Refuse délibérément les décisions automatiques affectant l'environnement système (pas de copie auto clipboard). Philosophie rare dans un écosystème où la plupart des apps automatisent "pour simplifier" mais créent des surprises indésirables.

**Innovation :** Respect total de l'environnement système et workflow utilisateur - l'utilisateur décide QUAND copier.

**Validation :** Pattern validé conceptuellement - évite la frustration "j'avais copié autre chose pendant la transcription".

**4. Quick Capture Spécialisé pour Prompts IA**

Design optimisé pour captures ultra-courtes (30sec-2min) vs dictée générique longue. Permet des simplifications radicales (effacement auto, pas d'historique, workflow linéaire) impossibles avec un use case générique.

**Innovation :** Spécialisation extrême pour un use case précis = 10x plus rapide qu'une solution générique pour ce cas d'usage.

**Validation :** Même si les utilisateurs veulent parfois de la dictée longue, l'app fonctionne également - la spécialisation n'exclut pas, elle optimise.

### Market Context & Competitive Landscape

**Recherche de Solutions Existantes :**
Aucune solution trouvée combinant :
- Transcription locale (privacy-first)
- Ghost mode / background invisible
- Optimisation pour prompts IA courts
- Workflow keyboard-first power user

**Solutions Existantes et Leurs Limitations :**

| Solution | Approche | Limitations |
|----------|----------|-------------|
| Google Docs Voice Typing | Cloud-based, intégré navigateur | Nécessite Google Docs ouvert, envoie données au cloud, pas de mode background |
| Apple Dictation | Cloud/on-device, intégré OS | Envoie données au cloud (mode extended), pas d'UI review rapide, contrôle limité |
| Dragon NaturallySpeaking | Desktop lourd, cloud optionnel | Lourd (>500MB), UI intrusive, conçu pour dictée longue, onéreux |
| Whisper Desktop wrappers | Local, open-source | UX basique, pas d'optimisation prompts IA, pas de ghost mode |

**Espace d'Innovation Identifié :**
Combinaison unique de privacy (local AI), UX power user (ghost mode + keyboard-first), et spécialisation use case (prompts IA courts). Aucune solution sur le marché ne combine ces trois axes.

### Validation Approach

**Innovation 1 : Privacy-First Local AI**
- **Méthode :** Tests comparatifs whisper.cpp vs APIs cloud sur 20+ prompts représentatifs
- **Critère de succès :** ≥90% exactitude en environnement calme
- **Statut :** Pré-validé par tests utilisateur, confirme viabilité technique
- **Risque :** Environnements très bruyants peuvent dégrader qualité
- **Mitigation :** Indicateur qualité environnement (Phase 2) pour prévenir utilisateur

**Innovation 2 : Ghost Mode UX**
- **Méthode :** Usage réel pendant 1 semaine avec workflow quotidien
- **Critère de succès :** Workflow devient réflexe, pas de friction ressentie
- **Statut :** À valider dans MVP
- **Risque :** UX trop invisible, utilisateur oublie que l'app existe
- **Mitigation :** Notifications post-transcription + indicateur system tray

**Innovation 3 : Contrôle Manuel Clipboard**
- **Méthode :** Observation comportementale sur 1 mois d'usage
- **Critère de succès :** Zéro frustration "j'avais copié autre chose"
- **Statut :** Pattern validé conceptuellement
- **Risque :** Utilisateur trouve ça moins "magique" que l'auto-copie
- **Mitigation :** Peut activer auto-copie optionnelle si demandé

**Innovation 4 : Quick Capture Spécialisé**
- **Méthode :** Usage quotidien 5-10x/jour pour prompts IA
- **Critère de succès :** Gain de temps ressenti vs typing manuel
- **Statut :** À valider dans usage réel
- **Risque :** Besoin de dictée longue émerge et app mal optimisée
- **Mitigation :** L'app fonctionne aussi pour dictée longue, juste moins optimisée (pas de blocker)

### Risk Mitigation

**Si Innovations Échouent :**

**Fallback Strategy - Simplification Progressive :**

1. **Si Ghost Mode trop complexe/invisible :**
   - Revenir à fenêtre traditionnelle minimisable
   - Garder raccourcis globaux pour lancement rapide
   - Impact minimal sur fonctionnalités core

2. **Si Privacy-First (whisper.cpp local) insuffisant :**
   - Ajouter option API cloud (Google Speech, OpenAI Whisper API)
   - Garder local comme défaut, cloud comme option
   - Préserver philosophy privacy-first avec choix utilisateur

3. **Si Contrôle Manuel Clipboard frustrant :**
   - Ajouter toggle "auto-copy" dans settings
   - Garder manuel comme défaut (philosophie respect environnement)
   - Permettre personnalisation selon préférence

4. **Si Quick Capture insuffisant pour dictée longue :**
   - App fonctionne déjà pour dictée longue
   - Ajouter historique optionnel si vraiment nécessaire
   - Garder effacement auto comme défaut, historique comme option

**Core Minimum Viable (si toutes innovations échouent) :**
- Enregistrement audio fonctionnel
- Transcription locale whisper.cpp
- Copie vers clipboard
- UI desktop minimale

Ce core reste viable même si toutes les innovations UX sont simplifiées. Les innovations sont des **amplificateurs d'expérience**, pas des **blockers critiques**.

## Desktop App Specific Requirements

### Project-Type Overview

**vocal-note-taker** est une application desktop native conçue pour s'intégrer étroitement avec l'environnement système tout en restant légère et non-intrusive. L'approche desktop permet une intégration profonde (raccourcis globaux, clipboard système, notifications) impossible avec une webapp, tout en garantissant un fonctionnement 100% offline et privacy-first.

**Justification du choix Desktop App :**
- **Raccourcis globaux système** - essentiels pour workflow sans friction
- **Ghost mode / background persistant** - l'app reste disponible sans consommer d'attention visuelle
- **Processing local lourd** - whisper.cpp nécessite exécution native pour performance
- **Privacy total** - aucune dépendance cloud, données restent locales
- **Intégration clipboard** - copie système native instantanée

### Platform Support

**Plateformes Cibles :**

**MVP (Phase 1) :**
- **Ubuntu 22.04+** - Plateforme prioritaire
- Support GNOME/KDE desktop environments
- Architecture x86_64

**Post-MVP (Phase 2) :**
- **macOS 12+** (Monterey et supérieur)
- Support Apple Silicon (M1/M2/M3) + Intel
- Pas de portage Windows prévu

**Approche Cross-Platform :**
- Framework : **Tauri** pour abstraction OS-agnostic
- Backend Rust identique sur tous les OS
- Adaptations spécifiques OS minimales (raccourcis globaux, notifications)
- Pas de distinction fonctionnelle majeure Ubuntu/Mac
- Rust compile nativement pour chaque plateforme

**Distribution :**
- Ubuntu : Package `.deb` via `apt` ou téléchargement direct
- macOS : `.dmg` ou `.app` bundle (Phase 2)
- Pas de store distribution prévue (pas App Store, pas Snap Store)

### System Integration

**Intégrations Système Critiques (MVP) :**

**1. Raccourcis Clavier Globaux**
- Raccourci global système pour lancer transcription (ex: Ctrl+Shift+Space)
- Fonctionnement même quand app en arrière-plan
- Configuration via fichier config (pas UI settings pour MVP)
- Format config : JSON ou YAML simple
- Ubuntu : utilise X11/Wayland APIs pour capture globale
- macOS : utilise Cocoa APIs (Phase 2)

**2. Clipboard Système**
- Copie native vers clipboard système
- Pas de gestion multi-clipboard complexe
- Simple copie texte brut (pas de formatting)
- Fonctionne avec Ctrl+V standard dans toute app cible

**3. Notifications Système**
- Notification Ubuntu native (libnotify) post-transcription
- Notification actionnable : clic ramène app au premier plan
- Pas de notification sounds (silencieux par défaut)
- macOS : Notification Center (Phase 2)

**4. Background / Ghost Mode**
- App reste en exécution background après fermeture fenêtre
- Pas de quit complet, juste minimisation
- Process Tauri + Rust backend persistent en mémoire
- Consommation RAM idle : <100MB cible (architecture Rust optimisée)

**Intégrations NON Incluses dans MVP :**
- ❌ System tray icon (pas nécessaire pour MVP)
- ❌ Démarrage automatique avec OS
- ❌ Integration avec desktop search (Spotlight, GNOME Search)
- ❌ Quick Actions / Share extensions
- ❌ Raccourcis configurables via UI (fichier config suffit)

### Update Strategy

**Approche MVP - Simplicité Maximale :**

**Update Manuel :**
- Pas de système auto-update dans MVP
- Utilisateur re-télécharge nouvelle version manuellement
- Réinstalle package .deb (Ubuntu) ou .dmg (Mac)
- Documentation changelog dans GitHub releases

**Rationale :**
- Usage personnel (1 utilisateur) = pas besoin d'update push
- Évite complexité technique significative (signature, delta updates, rollback)
- Focus développement sur features core, pas infrastructure update

**Post-MVP (si besoin émerge) :**
- Check version au démarrage avec notification "nouvelle version disponible"
- Lien direct vers page download GitHub
- Toujours update manuel, jamais auto-update silencieux

**Versioning :**
- Semantic versioning (SemVer) : v1.0.0, v1.1.0, v2.0.0
- Version affichée dans UI (About dialog)
- Breaking changes = major version bump

### Offline Capabilities

**100% Fonctionnement Offline - Requirement Critique :**

**Aucune Dépendance Réseau :**
- ✅ Transcription locale via whisper.cpp (pas d'API calls)
- ✅ Tout le processing se fait on-device
- ✅ Pas de telemetry, analytics, ou crash reporting cloud
- ✅ Pas de licence online check ou activation serveur

**Use Cases Offline Validés :**
- Utilisation dans avion sans WiFi : ✅ Fonctionne
- Utilisation dans train tunnel : ✅ Fonctionne
- Réseau entreprise restrictif : ✅ Fonctionne
- Pas de connexion internet du tout : ✅ Fonctionne

**Données Locales Uniquement :**
- Modèles whisper.cpp stockés localement (~3GB pour modèle large)
- Fichiers audio temporaires (WAV) sur disque local
- Configuration utilisateur en local (JSON/YAML)
- Aucun cloud storage, aucun sync

**Implications Installation :**
- Package .deb inclut tous binaires nécessaires
- whisper.cpp modèle large téléchargé une fois à l'installation
- Post-installation : 0 dépendance réseau

### Technical Architecture Considerations

**Stack Technique Desktop :**

**Framework & Backend Unifié :**
- **Tauri + Rust** - Framework desktop moderne avec backend Rust intégré
- HTML/CSS/JS pour UI minimale
- Webview système natif (pas Electron = plus léger)
- Backend Rust pour orchestration et audio capture

**Audio Capture :**
- **cpal** - Audio capture cross-platform en Rust
- Alternative : **rodio** pour capture et processing audio
- Support natif Ubuntu (ALSA/PulseAudio) et macOS (CoreAudio)

**AI Processing :**
- **whisper-rs** - Bindings Rust pour whisper.cpp
- Modèle **large** (~3GB) - qualité maximale, hardware puissant requis
- Exécution CPU optimisée (tourne bien sur hardware utilisateur)
- Intégration native sans subprocess ni IPC complexe

**Architecture Unifiée :**
- Tout en Rust - pas de communication inter-process
- Tauri commands appellent directement fonctions Rust backend
- Pas de serveur HTTP/WebSocket
- Communication directe en mémoire (zero-copy quand possible)

**Stockage Local :**
- Fichiers audio temp : `/tmp/vocal-note-taker/` (cleaned après transcription)
- Config : `~/.config/vocal-note-taker/config.yaml`
- Logs (optionnel) : `~/.local/share/vocal-note-taker/logs/`

**Sécurité :**
- Pas de network calls = surface d'attaque minimale
- Permissions système : microphone access uniquement
- Pas de sudo/root nécessaire
- Données audio jamais uploadées nulle part

### Implementation Considerations

**Défis Techniques Anticipés :**

**1. Raccourcis Globaux Cross-Platform**
- **Challenge :** APIs différentes Ubuntu (X11/Wayland) vs macOS (Cocoa)
- **Solution :** Tauri plugin global-hotkey ou bibliothèque Rust dédiée (global-hotkey crate)
- **Fallback :** Si global hotkeys échouent, raccourcis in-app uniquement

**2. Background Mode Persistant**
- **Challenge :** Empêcher OS de killer app en background
- **Solution Ubuntu :** Systemd service optionnel ou process daemon
- **Solution macOS :** LSUIElement=true pour app sans dock icon (Phase 2)

**3. Performance Whisper.cpp Large Model**
- **Challenge :** Modèle large (~3GB) nécessite hardware puissant
- **Solution :** Prérequis système clairement documentés
- **Validation :** Hardware utilisateur confirmé compatible (tourne bien)
- **Pas de fallback** vers modèle plus petit - qualité maximale prioritaire

**4. Audio Capture Cross-Platform (cpal)**
- **Challenge :** Audio capture différente selon OS et desktop environment
- **Solution :** cpal supporte ALSA/PulseAudio (Ubuntu) et CoreAudio (macOS)
- **Fallback :** Tests sur GNOME, KDE, Wayland pour valider compatibilité

**Requirements Système Minimum :**
- **CPU :** Modern multi-core (4+ cores recommandé pour modèle large)
- **RAM :** 8GB minimum (16GB recommandé)
- **Stockage :** 5GB disponible (modèle large + app)
- **OS :** Ubuntu 22.04+ avec ALSA/PulseAudio, ou macOS 12+ (Phase 2)

**Priorités Implémentation MVP :**
1. Ubuntu support solide et testé
2. Transcription qualité ≥90% validée avec modèle large
3. Raccourcis globaux fonctionnels
4. macOS et features avancées = post-MVP

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Problem-Solving MVP

L'approche choisie est un **Problem-Solving MVP** focalisé sur la résolution d'un problème spécifique : capturer rapidement des prompts vocaux pour IA avec qualité professionnelle et privacy totale. Le MVP livre la valeur core minimale nécessaire pour valider l'usage quotidien.

**Philosophie :** Simple MVP avec lean scope adapté à un projet personnel développé par une seule personne avec assistance IA.

**Resource Requirements:**
- **Équipe :** 1 développeur (Tamles) + assistance IA pour accélération
- **Timeline :** 2 semaines développement MVP
- **Compétences :** Rust + Tauri + développement assisté IA (bon niveau)
- **Infrastructure :** Local development (Ubuntu), pas de cloud/serveurs

### MVP Feature Set - Référence

**Note :** Le scope MVP complet est défini dans la section **"Product Scope > MVP - Minimum Viable Product (2 Semaines)"** ci-dessus.

**Résumé des Must-Haves (7 features critiques) :**
1. Enregistrement audio fonctionnel
2. Transcription locale whisper.cpp (modèle large)
3. UI minimale desktop
4. Workflow linéaire simple
5. Copie clipboard optimisée
6. Feedback visuel basique
7. Package installable Ubuntu

**Validation MVP :** Workflow complet en <15 secondes, qualité ≥90%, utilisable quotidiennement.

### Post-MVP Features - Référence

**Note :** Les phases de croissance sont définies dans la section **"Product Scope > Growth Features"** et **"Vision (Future)"** ci-dessus.

**Phase 2 (Semaines 3-4) :** Power user enhancements + Polish
**Phase 3 (Version 2.0) :** Intelligence audio avancée + Ghost mode complet

### Risk Mitigation Strategy

**Technical Risks:**

**Risque #1 : Courbe d'apprentissage Rust**
- **Probabilité :** Moyenne
- **Impact :** Moyen (peut ralentir développement initial)
- **Mitigation :** Développement assisté par IA (Claude/Copilot) pour accélération Rust. Architecture unifiée (pas de Python subprocess) simplifie le code. Focus sur MVP minimal pour limiter complexité.

**Risque #2 : Performance whisper.cpp modèle large**
- **Probabilité :** Faible (hardware validé)
- **Impact :** Moyen
- **Mitigation :** Hardware utilisateur confirmé compatible. Documentation prérequis système clairs. Pas de fallback modèle plus petit = qualité prioritaire.

**Risque #3 : Raccourcis globaux cross-platform**
- **Probabilité :** Moyenne (APIs OS différentes)
- **Impact :** Moyen (feature importante mais pas blocker)
- **Mitigation :** Tauri plugin global-hotkey. Si échec, raccourcis in-app uniquement pour MVP.

**Market Risks:**

**Risque #1 : Adoption réelle après 1 semaine**
- **Probabilité :** Faible (besoin validé personnellement)
- **Impact :** Élevé (si pas utilisé quotidiennement = échec)
- **Validation :** Usage forcé quotidien pendant 1 semaine. Suivi fréquence réelle vs cible 5-10x/jour.

**Risque #2 : Qualité transcription insuffisante**
- **Probabilité :** Faible (tests pré-validés)
- **Impact :** Critique (blocker si <90%)
- **Validation :** Tests sur 20+ prompts représentatifs avant déclaration MVP terminé.

**Resource Risks:**

**Risque #1 : Timeline 2 semaines dépassée**
- **Probabilité :** Moyenne (développement assisté IA peut accélérer ou ralentir)
- **Impact :** Moyen (réévaluation ROI si >3 semaines)
- **Contingency :** Réduire scope MVP si nécessaire. Features minimum absolu : enregistrement + transcription + copie clipboard. Tout le reste = nice-to-have.

**Risque #2 : Blockers techniques imprévus**
- **Probabilité :** Faible
- **Impact :** Élevé
- **Contingency :** Fallback vers architecture plus simple si nécessaire. Sacrifice UX pour garantir fonctionnalité core. Architecture unifiée Rust réduit les points de défaillance.

**Success Criteria Alignment:**

Le scoping MVP est directement aligné avec les critères de succès définis :
- ✅ Timeline 2 semaines respectée via lean scope
- ✅ Fréquence 5-10x/jour validable avec MVP fonctionnel
- ✅ Workflow <15 sec atteignable avec features MVP
- ✅ Qualité ≥90% garantie par modèle large whisper.cpp

## Functional Requirements

### Audio Recording

- **FR1:** User can initiate audio recording via button click
- **FR2:** User can initiate audio recording via global keyboard shortcut
- **FR3:** User can stop audio recording via button click or keyboard shortcut release
- **FR4:** System can capture audio from system microphone input
- **FR5:** System can display recording timer showing elapsed time in seconds
- **FR6:** System can display visual recording indicator (REC icon) during active recording
- **FR7:** System can display real-time audio waveform visualization during recording
- **FR8:** System can save recorded audio as temporary WAV file (16kHz mono)

### Transcription Processing

- **FR9:** System can transcribe recorded audio using local whisper.cpp model (large)
- **FR10:** System can process transcription entirely offline without network dependency
- **FR11:** System can display transcription progress indicator to user
- **FR12:** System can complete transcription and display results
- **FR13:** System can handle transcription errors gracefully with clear error messages
- **FR14:** System can clean up temporary audio files after successful transcription

### Text Display & Management

- **FR15:** User can view complete transcribed text in readable format
- **FR16:** System can display transcribed text without truncation or scrolling when text fits viewport
- **FR17:** User can visually scan transcribed text for accuracy verification
- **FR18:** System can automatically clear previous transcription when starting new recording
- **FR19:** System can maintain simple linear workflow (no history management)

### Clipboard Integration

- **FR20:** User can copy transcribed text to system clipboard via button click
- **FR21:** User can copy transcribed text to system clipboard via Enter keyboard shortcut
- **FR22:** System can automatically focus copy button after transcription completes
- **FR23:** System can display visual confirmation feedback when text is copied ("✓ Copié!")
- **FR24:** System can copy plain text format (no rich formatting)
- **FR25:** User controls when clipboard copy occurs (manual trigger, not automatic)

### System Integration

- **FR26:** System can register and respond to global keyboard shortcuts while in background
- **FR27:** System can continue running in background after window closure (ghost mode)
- **FR28:** System can minimize to background without terminating processes
- **FR29:** System can display system notification when transcription is complete
- **FR30:** User can click notification to bring application to foreground
- **FR31:** System can integrate with Ubuntu notification system (libnotify)
- **FR32:** System can maintain process persistence in memory (Rust backend + Tauri)

### Configuration Management

- **FR33:** User can configure global keyboard shortcuts via configuration file
- **FR34:** System can load configuration from local file (JSON or YAML format)
- **FR35:** System can store configuration in user config directory (~/.config/vocal-note-taker/)
- **FR36:** System can apply configuration changes on application restart
- **FR37:** System can use default configuration if custom config not found

### Application Lifecycle

- **FR38:** User can install application via .deb package on Ubuntu
- **FR39:** System can function entirely offline without internet connection
- **FR40:** System can start application from Ubuntu applications menu or command line
- **FR41:** System can display application version in UI
- **FR42:** User can quit application completely via menu or shortcut
- **FR43:** System can maintain RAM consumption under 100MB when idle

### Error Handling & Recovery

- **FR44:** System can detect and report microphone access errors
- **FR45:** System can detect and report whisper.cpp processing failures
- **FR46:** System can recover gracefully from recording interruptions
- **FR47:** System can provide clear actionable error messages to user
- **FR48:** System can continue operating after non-critical errors

---

**Total Functional Requirements:** 48 FRs across 8 capability areas

**Coverage Validation:**
- ✅ All MVP features from Product Scope covered
- ✅ All user journey capabilities included
- ✅ Desktop app system integration requirements captured
- ✅ Error handling and lifecycle management included
- ✅ Configuration and offline capabilities documented
- ✅ Each FR is testable and implementation-agnostic

**Capability Contract:**
This FR list represents the complete capability inventory for vocal-note-taker MVP. Any capability not listed here will not exist in the final product. UX design, architecture, and implementation will be scoped exclusively to these 48 requirements.

## Non-Functional Requirements

### Performance

**NFR-PERF-1: Workflow Total Response Time**
- Complete workflow (raccourci clavier → transcription → copie) must complete in less than 15 seconds for 60 seconds of audio input
- Measurement: End-to-end timer from shortcut press to clipboard copy confirmation

**NFR-PERF-2: Transcription Latency**
- Audio transcription must complete within 30 seconds for 60 seconds of recorded audio
- Quality prioritized over speed - acceptable to take longer if quality ≥90%
- System notification allows user to continue other tasks during processing

**NFR-PERF-3: UI Responsiveness**
- User interface must respond to interactions (clicks, keyboard input) within 100ms
- No perceptible freeze or lag during recording or transcription
- UI remains interactive during background processing

**NFR-PERF-4: Memory Consumption**
- Application idle memory consumption target: <200MB RAM
- Not strict requirement but must remain reasonably low given always-active nature
- Memory leaks prevented through proper cleanup of temporary resources

**NFR-PERF-5: Application Startup**
- Startup time not critical (application launched once and remains in background)
- Acceptable to take several seconds on initial launch
- Fast recovery from background state to foreground (<500ms)

### Usability

**NFR-USA-1: Cognitive Load Minimization**
- Interface must be instantaneously readable without mental effort
- Visual hierarchy clear with no ambiguity on available actions
- State transitions obvious (idle → recording → transcribing → ready)

**NFR-USA-2: Quick Quality Verification**
- User can visually scan transcribed text for accuracy in 2-3 seconds
- Text display optimized for rapid comprehension
- Clear visual separation between UI controls and transcription content

**NFR-USA-3: Friction-Free Workflow**
- Maximum 3 user actions required for complete workflow (shortcut → speak → copy)
- No unnecessary confirmation dialogs or interruptions
- Linear workflow with automatic progression between states

**NFR-USA-4: Keyboard-First Interaction**
- All critical actions accessible via keyboard shortcuts
- No mouse required for primary workflow
- Clear visual indication of focused elements for keyboard navigation

**NFR-USA-5: Feedback Clarity**
- Continuous visual feedback during recording (waveform, timer, REC indicator)
- Immediate confirmation feedback for user actions (✓ Copié!, error messages)
- System state always visibly apparent to user

### Reliability

**NFR-REL-1: Crash Tolerance**
- Application crash rate must be less than 1 occurrence per week of daily usage
- Crashes are acceptable but not frequent
- Graceful degradation preferred over hard failures

**NFR-REL-2: Uptime & Restart Requirements**
- Application should support multiple days of continuous operation without restart
- Daily restart acceptable if necessary for stability
- No data corruption if application runs for extended periods

**NFR-REL-3: Data Loss Tolerance**
- Loss of in-progress audio recording acceptable if application crashes
- No persistent data beyond current session (workflow is ephemeral by design)
- Temporary files cleaned up on next application start if crash occurred

**NFR-REL-4: Error Recovery**
- Application must recover gracefully from non-critical errors
- Clear, actionable error messages displayed to user
- System continues operating after recoverable errors (microphone busy, transcription timeout)

**NFR-REL-5: System Stability**
- Zero conflicts with other applications running on system
- No interference with system clipboard or other shared resources
- Predictable behavior in multi-application environment

### Security & Privacy

**NFR-SEC-1: Network Isolation**
- Zero network calls during normal operation
- All transcription processing occurs locally on-device
- No telemetry, analytics, or crash reporting to external servers

**NFR-SEC-2: Data Privacy**
- Voice data never leaves local machine
- Audio recordings never uploaded or transmitted
- Transcribed text remains local until user explicitly copies to clipboard

**NFR-SEC-3: Temporary Data Cleanup**
- Temporary audio files (WAV) deleted immediately after successful transcription
- No audio recording persistence beyond active session
- Cleanup occurs even if transcription fails

**NFR-SEC-4: Minimal Permissions**
- Only microphone access permission required
- No root/sudo privileges needed for operation
- No access to user files outside application directories

**NFR-SEC-5: Configuration Security**
- Configuration files stored in user-specific directories (~/.config/)
- No sensitive credentials or secrets stored
- Configuration in plain text (JSON/YAML) for transparency

### Maintainability

**NFR-MAINT-1: Code Clarity**
- Code must be sufficiently clear for future modifications
- Meaningful variable/function names and logical structure
- Minimal code for future developer (Tamles) to understand after long pauses

**NFR-MAINT-2: Modular Architecture**
- Clear separation between Tauri frontend, Rust backend, and whisper-rs integration
- Changes to one module minimally impact others
- Well-defined interfaces between components

**NFR-MAINT-3: Documentation Minimum**
- Basic README with setup and usage instructions
- Architecture overview diagram or description
- Troubleshooting guide for common issues

**NFR-MAINT-4: Maintenance Time Budget**
- Post-MVP maintenance effort: maximum 2-4 hours per month
- Bug fixes and small improvements only
- No ongoing operational overhead (no servers, no cloud services)

**NFR-MAINT-5: Dependency Management**
- Minimal external dependencies to reduce maintenance burden
- Clear documentation of required dependencies (Rust toolchain, Tauri, whisper-rs, cpal)
- Stable, well-maintained libraries preferred over cutting-edge
- Cargo.toml lockfile pour reproducibilité builds

---

**NFR Coverage Summary:**

- **Performance:** 5 requirements - workflow speed, transcription latency, UI responsiveness, memory, startup
- **Usability:** 5 requirements - cognitive load, quality verification, workflow friction, keyboard-first, feedback
- **Reliability:** 5 requirements - crash tolerance, uptime, data loss, error recovery, system stability
- **Security & Privacy:** 5 requirements - network isolation, data privacy, cleanup, minimal permissions, config security
- **Maintainability:** 5 requirements - code clarity, modularity, documentation, time budget, dependencies

**Total:** 25 non-functional requirements across 5 quality attribute categories

**Categories Excluded:** Scalability (single user), Accessibility (personal use), Integration (standalone app)
