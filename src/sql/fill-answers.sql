INSERT INTO message (text, answer_id) VALUES (
  'Hello, dood',
  (SELECT id FROM message WHERE text = 'Hello, it is main topic' LIMIT 1)
);

INSERT INTO message (text, answer_id) VALUES (
  'Not bad, mr.',
  (SELECT id FROM message WHERE text = 'Hello, it is main topic' LIMIT 1)
);

INSERT INTO message (text, answer_id) VALUES (
  'Wow, I will stay here forever',
  (SELECT id FROM message WHERE text = 'Fortnite topic' LIMIT 1)
);

INSERT INTO message (text, answer_id) VALUES (
  'I loocked, but it is just a dot :(',
  (SELECT id FROM message WHERE text = 'Loocked this ->' LIMIT 1)
);

INSERT INTO message (text, answer_id) VALUES (
  'Wow, you were the first',
  (SELECT id FROM message WHERE text = 'I loocked, but it is just a dot :(' LIMIT 1)
);

INSERT INTO message (text, answer_id) VALUES (
  'When will there be a banner with Eola?',
  (SELECT id FROM message WHERE text = 'Genshin Impact' LIMIT 1)
);

INSERT INTO message (text, answer_id) VALUES (
  '....Never.....',
  (SELECT id FROM message WHERE text = 'When will there be a banner with Eola?' LIMIT 1)
);
