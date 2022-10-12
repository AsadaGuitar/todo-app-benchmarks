CREATE TABLE IF NOT EXISTS user_accounts(
    id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    close BOOLEAN DEFAULT FALSE NOT NULL,
    create_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    modify_at TIMESTAMP WITH TIME ZONE,
    close_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY(id)
);

INSERT INTO user_accounts(id, name, password) VALUES 
(
    'mNXpMugRoTXI',
    'user_1',
    'user_1_password'
), 
(
    'PfTTSONBySjb',
    'user_2',
    'user_2_password'
),
(
    'QHkaQGYwuRFc',
    'user_3',
    'user_3_password'
);