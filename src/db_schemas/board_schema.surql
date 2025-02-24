-- Define the `person` table
DEFINE TABLE IF NOT EXISTS person SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE person TYPE string;
DEFINE FIELD IF NOT EXISTS alias ON TABLE person TYPE option<string>;
DEFINE FIELD IF NOT EXISTS description ON TABLE person TYPE option<string>;
DEFINE FIELD IF NOT EXISTS date_of_birth ON TABLE person TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS address ON TABLE person TYPE option<string>;

-- Define the `location` table
DEFINE TABLE IF NOT EXISTS location SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE location TYPE string;
DEFINE FIELD IF NOT EXISTS coordinates ON TABLE location TYPE option<geometry>;

-- Define the `organization` table
DEFINE TABLE IF NOT EXISTS organization SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE organization TYPE string;
DEFINE FIELD IF NOT EXISTS description ON TABLE organization TYPE option<string>;

-- Define the `event` table
DEFINE TABLE IF NOT EXISTS event SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE event TYPE string;
DEFINE FIELD IF NOT EXISTS description ON TABLE event TYPE option<string>;

-- Define the `evidence` table
DEFINE TABLE IF NOT EXISTS evidence SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE evidence TYPE string;
DEFINE FIELD IF NOT EXISTS description ON TABLE evidence TYPE option<string>;
DEFINE FIELD IF NOT EXISTS evidence_type ON TABLE evidence TYPE string;
DEFINE FIELD IF NOT EXISTS location ON TABLE evidence TYPE option<record<location>>;

-- Define the `related_to` edge (generic relationship)
DEFINE TABLE IF NOT EXISTS related_to TYPE RELATION;
-- TYPE record<person|location|organization|event|evidence>;
-- TYPE record<person|location|organization|event|evidence>;

-- Define the `at` edge (person/event at a location)
DEFINE TABLE IF NOT EXISTS at_location TYPE RELATION;
-- TYPE record<person|event>;
-- TYPE record<location>;

-- Define the `member_of` edge (person belongs to an organization)
DEFINE TABLE IF NOT EXISTS member_of TYPE RELATION;
-- TYPE record<person>;
-- TYPE record<organization>;

-- Define the `involved_in` edge (person involved in an event)
DEFINE TABLE IF NOT EXISTS involved_in TYPE RELATION;
-- TYPE record<person>;
-- TYPE record<event>;

-- Define the `found_at` edge (evidence found at a location)
DEFINE TABLE IF NOT EXISTS found_at TYPE RELATION;
-- TYPE record<evidence>;
-- TYPE record<location>;

-- Define the `related_evidence` edge (evidence related to an entity)
DEFINE TABLE IF NOT EXISTS has_related_evidence TYPE RELATION;
-- TYPE record<person|event|organization>;
-- TYPE record<evidence>;

-- Define the `linked_to` edge (person linked to person or organization linked to organization)
DEFINE TABLE IF NOT EXISTS linked_to TYPE RELATION;
-- TYPE record<person|organization>;
-- TYPE record<person|organization>;


-- ** TEST CREATIONS **
-- ** PERSON **
CREATE person:devon SET name = 'Devon', alias = 'el ligaligle', description = "Devil's Advocate", date_of_birth = d"1981-04-23", address='California';
CREATE person:john_doe SET name = 'John Doe', alias = 'el desconocido', description = "Nobody knows him", date_of_birth = d"1984-03-02", address='California';
CREATE person:caitlyn_kiramman SET name = 'Caitlyn Kiramman', alias = 'la fresa', description = 'Sniper', address='Piltover';
CREATE person:max_tech SET name = 'Max Tech', alias = 'el macaco', description = 'Tech smugglers';

-- ** LOCATION **
CREATE location:nyc_soho SET name = 'Soho, New York', coordinates = { type: "Point", coordinates: [40.7246, 74.0019] };
CREATE location:nyc_guggenheim SET name = 'Guggenheim, New York', coordinates = { type: "Point", coordinates: [40.7830, 73.9590] };
CREATE location:nyc_conney_island SET name = 'Conney Island, New York', coordinates = { type: "Point", coordinates: [40.5755, 73.9707] };
CREATE location:watergate_office SET name = 'Watergate, Washington, DC', coordinates = { type: "Point", coordinates: [38.8996, 77.0556] };

-- ** ORGANIZATION **
CREATE organization:legal_eagle SET name = 'Legal Eagle', description = 'Advocates of the devil';
CREATE organization:spectre SET name = 'Spectre', description = 'Global Terrorism';
CREATE organization:zetas SET name = 'Zetas', description = 'Narc terrorists';
CREATE organization:interpol SET name = 'Interpol', description = 'International police';

-- ** EVENTS **
CREATE event:shooting SET name = 'Shooting', description = 'shooting';
CREATE event:break_in SET name = 'Break in', description = 'breaking into a property';
CREATE event:car_chase SET name = 'Car Chase', descripotion = 'cars chaising';

-- ** EVIDENCE **
CREATE evidence:ar_15 SET name = "AR-15", description = 'Fired weapon', evidence_type = "", location = location:nyc_soho;
CREATE evidence:hextech_sniper_rifle SET name = "HexTech Sniper Rifle", description = 'Fired weapon', evidence_type = "", location = location:nyc_conney_island;
CREATE evidence:footprints_on_sand SET name = "Footprints on sand", description = 'MX size 6', evidence_type = "", location = location:nyc_guggenheim;
CREATE evidence:dna_samples SET name = "DNA Samples", description = "blood and hair", evidence_type = "", location = location:watergate_office;
CREATE evidence:female_corpse SET name = "Female corpse", description = "Female death", evidence_type = "";

-- ** PERSON **
CREATE person:devon SET name = 'Devon', alias = 'el ligaligle', description = "Devil's Advocate", date_of_birth = d"1981-04-23", address='California';
CREATE person:john_doe SET name = 'John Doe', alias = 'el desconocido', description = "Nobody knows him", date_of_birth = d"1984-03-02", address='California';
CREATE person:caitlyn_kiramman SET name = 'Caitlyn Kiramman', alias = 'la fresa', description = 'Sniper', address='Piltover';
CREATE person:max_tech SET name = 'Max Tech', alias = 'el macaco', description = 'Tech smugglers';

-- ** LOCATION **
CREATE location:nyc_soho SET name = 'Soho, New York', coordinates = <geometry>{ type: "Point", coordinates: [40.7246, 74.0019] };
CREATE location:nyc_guggenheim SET name = 'Guggenheim, New York', coordinates = <geometry>{ type: "Point", coordinates: [40.7830, 73.9590] };
CREATE location:nyc_conney_island SET name = 'Conney Island, New York', coordinates = <geometry>{ type: "Point", coordinates: [40.5755, 73.9707] };
CREATE location:watergate_office SET name = 'Watergate, Washington, DC', coordinates = <geometry>{ type: "Point", coordinates: [38.8996, 77.0556] };

-- ** ORGANIZATION **
CREATE organization:legal_eagle SET name = 'Legal Eagle', description = 'Advocates of the devil';
CREATE organization:spectre SET name = 'Spectre', description = 'Global Terrorism';
CREATE organization:zetas SET name = 'Zetas', description = 'Narc terrorists';
CREATE organization:interpol SET name = 'Interpol', description = 'International police';

-- ** EVENTS **
CREATE event:shooting SET name = 'Shooting', description = 'shooting';
CREATE event:break_in SET name = 'Break in', description = 'breaking into a property';
CREATE event:car_chase SET name = 'Car Chase', descripotion = 'cars chaising';

-- ** SOME TEST RELATIONS **
RELATE person:caitlyn_kiramman->member_of->organization:interpol;
RELATE person:devon->member_of->organization:legal_eagle;
RELATE person:john_doe->member_of->organization:spectre;
RELATE organization:legal_eagle->linked_to->organization:spectre;
RELATE organization:zetas->linked_to->organization:legal_eagle;
RELATE event:shooting->at_location->location:nyc_guggenheim;
RELATE person:caitlyn_kiramman->involved_in->event:shooting;
RELATE evidence:ar_15->found_at->location:nyc_guggenheim;
RELATE event:break_in->at_location->location:watergate_office;
RELATE evidence:dna_samples->found_at->location:watergate_office;
RELATE evidence:dna_samples->related_to->person:john_doe;
RELATE evidence:female_corpse->found_at->location:soho;
RELATE organization:zetas->linked_to->location:nyc_conney_island;
RELATE organization:spectre->found_at->location:soho;
RELATE event:car_chase->at_location->location:soho;
RELATE evidence:footprints_on_sand->found_at->location:nyc_conney_island;

-- More Test Relationships

-- Devon's Involvement
RELATE person:devon->involved_in->event:break_in;  -- Devon was involved in the break-in.
RELATE person:devon->linked_to->person:john_doe; -- Devon has some connection to John Doe.

-- John Doe's Activities
RELATE person:john_doe->involved_in->event:shooting; -- John Doe was also involved in the shooting (maybe he was the target?).

-- Caitlyn's Investigation
RELATE person:caitlyn_kiramman->has_related_evidence->evidence:hextech_sniper_rifle; -- Caitlyn is examining the sniper rifle.
RELATE evidence:hextech_sniper_rifle->found_at->location:nyc_conney_island; -- The rifle was found at Coney Island
RELATE person:caitlyn_kiramman->at_location->location:nyc_conney_island;      -- Caitlyn was at Coney Island (investigating).
RELATE person:caitlyn_kiramman->at_location->location:nyc_guggenheim;      -- Caitlyn was also at the Guggenheim.

-- Organization Connections
RELATE organization:interpol->linked_to->event:shooting;  -- Interpol is investigating the shooting.
RELATE organization:spectre->has_related_evidence->evidence:dna_samples; -- Spectre is linked to the DNA evidence.
RELATE organization:legal_eagle->has_related_evidence->evidence:footprints_on_sand;  -- Legal Eagle is trying to use footprint

-- Event Connections
-- DNA evidence is linked to the break-in.
RELATE event:break_in->has_related_evidence->evidence:dna_samples;
RELATE event:shooting->has_related_evidence->evidence:ar_15;    -- The AR-15 is evidence in the shooting.

-- More Evidence Relationships
RELATE evidence:ar_15->related_to->person:john_doe;      -- The AR-15 is linked to John Doe (maybe he owns it?).
RELATE evidence:footprints_on_sand->related_to->person:max_tech; -- The footprints might belong to Max Tech.

-- Max Tech
RELATE person:max_tech->involved_in->event:car_chase;     -- Max Tech was involved in the car chase.
RELATE person:max_tech->member_of->organization:zetas; -- Max Tech is a member of the Zetas.
RELATE person:max_tech->at_location->location:nyc_conney_island;  -- Max Tech was at Coney Island.

-- Additional Relationships (more complex connections)
RELATE person:devon->related_to->evidence:footprints_on_sand; -- Devon is interested in the footprints (as a lawyer).
RELATE organization:legal_eagle->linked_to->event:break_in; -- Legal Eagle is representing someone involved in the break-in.
RELATE organization:interpol->has_related_evidence->evidence:female_corpse; --Interpol is investigating the corpse

-- Linking Locations and Events
RELATE event:car_chase->at_location->location:nyc_soho; -- Car chase also went through Soho.

-- More Person to Person Connections
RELATE person:john_doe->linked_to->person:max_tech;-- John and Max are connected.

-- Additional connections
RELATE person:john_doe->at_location->location:nyc_soho;
RELATE person:devon->at_location->location:watergate_office;
RELATE organization:interpol->linked_to->location:watergate_office;
RELATE evidence:ar_15->related_to->event:shooting;
RELATE evidence:hextech_sniper_rifle->related_to->event:shooting;
RELATE evidence:footprints_on_sand->related_to->event:car_chase;
RELATE evidence:female_corpse->related_to->event:shooting;

-- Linking back to organizations
RELATE organization:zetas->involved_in->event:car_chase;
RELATE organization:spectre->involved_in->event:shooting;

-- Extra connections
RELATE person:devon->has_related_evidence->evidence:ar_15;
RELATE person:max_tech->has_related_evidence->evidence:dna_samples;
