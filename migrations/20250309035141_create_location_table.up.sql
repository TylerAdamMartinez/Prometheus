CREATE TABLE location (
    location_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    altitude DOUBLE PRECISION,
    street_number TEXT,
    street_name TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT,
    country TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    bounding_box GEOMETRY(POLYGON, 4326), -- Bounding box as a spatial polygon
    location GEOMETRY(Point, 4326) NOT NULL, -- Latitude/Longitude as a PostGIS Point
    time_zone TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    deactivated_at TIMESTAMPTZ,
    is_public BOOLEAN DEFAULT false,
    notes TEXT
);
