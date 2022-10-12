CREATE TABLE IF NOT EXISTS tasks(
    id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    details TEXT,
    close BOOLEAN DEFAULT FALSE NOT NULL,
    create_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    modify_at TIMESTAMP WITH TIME ZONE,
    close_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY(id),
    FOREIGN KEY(user_id) 
        REFERENCES user_accounts(id) 
);

INSERT INTO tasks(id, user_id, title, details) VALUES
(
    'sEUMhpeeNOyu',
    'mNXpMugRoTXI',
    'Reservations for Year-End Party',
    'Book a year-end party for your company. Number of people: 20'
),
(
    'YByqkuBkOTOT',
    'PfTTSONBySjb',
    'Create an HP website',
    'Create a website site for Sample Inc.'
),
(
    'ncNQubXmBkLH',
    'QHkaQGYwuRFc',
    'Clean up the room',
    'I have a friend coming over tomorrow.'
);