# Battle of Culiacán (El Culiacanazo / Black Thursday)

> **Metadata**  
- `slug`: battle-of-culiacan  
- `title`: Battle of Culiacán – Mission Pack  
- `location`: Culiacán, Sinaloa, Mexico  
- `date`: 2019-10-17  
- `factions`: ["Sinaloa Cartel", "Mexican Armed Forces"]  
- `map_reference`: cul-urban-2019.json  
- `type`: Urban Warfare / Asymmetric Engagement  
- `difficulty`: Dynamic (scales based on media attention + time elapsed)  
- `civilian_presence`: High  
- `source`: [Wikipedia](https://en.wikipedia.org/wiki/Battle_of_Culiac%C3%A1n)

---

## Historical Overview

The **Battle of Culiacán** was a high-intensity urban conflict triggered by the attempted capture of **Ovidio Guzmán López**, son of El Chapo, by Mexican federal forces on October 17, 2019. It led to a rapid and overwhelming military-style response by the **Sinaloa Cartel**, effectively forcing the government to release him and retreat.

---

## Faction Profiles

### 🎯 **Sinaloa Cartel** (Playable)
- Strengths: Guerrilla tactics, urban dominance, real-time coordination
- Weaknesses: Fewer soldiers, limited air capabilities
- Units: Sicarios, Narco-Tech Trucks, Drone Blockers, Hostage Squads
- Special Abilities: Roadblock Deploy, Family Pressure, Propaganda Push

### 🛡️ **Mexican Military** (AI-controlled)
- Strengths: Numbers, armored support, helicopters
- Weaknesses: Slower response, limited intel
- Units: Infantry, Tactical Convoy, Air Support, Extraction Teams
- Special Conditions: Must protect civilians and avoid escalation

---

## Mission Hooks

### 🔥 Mission 1: "Initial Raid"
- **Objective:** Delay or prevent the arrest of Ovidio Guzmán.
- **Trigger:** Military locates safehouse; initiate counter-operation.
- **Gameplay:** Defensive setup in residential zone. Minimize cartel casualties.

### 🛑 Mission 2: "Cut Off the Convoy"
- **Objective:** Block all exit points out of Culiacán.
- **Tools:** Deploy burning vehicles, spike strips, control intersections.
- **Win Condition:** Prevent Ovidio’s extraction within 15 minutes.

### 👪 Mission 3: "Apply Pressure"
- **Objective:** Force government de-escalation.
- **Method:** Target military housing, capture officers’ families (non-lethal).
- **Risk:** Civilian casualties increase national attention.

### 🕒 Mission 4: "Hold the Line"
- **Objective:** Maintain chaos long enough for political reversal.
- **Condition:** Survive waves of escalating military pressure.
- **Twist:** International observers arrive, raising scrutiny.

---

## Tactical Systems

- **Urban Control Grid:** Holding districts increases income and visibility.
- **Civilian Presence:** Collateral damage increases difficulty and reputation cost.
- **Media Meter:** Real-time reputation system—higher visibility brings pressure on both sides.
- **Asymmetric Morale:** Military morale drops with every public failure; cartel morale rises with perceived impunity.

---

## Design Notes

- Emphasis on **asymmetric balance**: small units, high-impact actions.
- Time-based pressure: every minute changes global perception.
- Support multiple perspectives in sandbox mode (e.g., switch sides for narrative framing).
- Sound design: real radio chatter, media headlines, drone ambient loops.
- Optionally include **real quotes or government statements** in briefing cutscenes.

---

## Files & Resources

- `culiacan_map_layout.json` – map grid with real zones and key routes  
- `ovidio_metadata.json` – unit data for HVT  
- `mission_scripts/culiacan_m1_initial_raid.gd` – first mission logic (Godot)  
- `audio/chatter_17oct_radio.ogg` – ambient sound loop  
- `images/ui_overlay_blackthursday.png` – mission briefing UI skin

