TRUNCATE TABLE "user" RESTART IDENTITY CASCADE;

INSERT INTO "user" (id, email, password) VALUES
('00000000-0000-0000-0000-000000000001', 'john.doe@example.com', '$2b$12$jodXlOL.vRegGN.PQAghqOtgkj9/XmU8SqaPvrDOcDcgEduj7griS'),
('00000000-0000-0000-0000-000000000002', 'jane.smith@example.com', '$2b$12$jodXlOL.vRegGN.PQAghqOtgkj9/XmU8SqaPvrDOcDcgEduj7griS');