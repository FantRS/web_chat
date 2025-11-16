-- MIGRATION FOR ROLE CREATION --

INSERT INTO roles (rolename)
VALUES 
    ('admin'), 
    ('moderator'), 
    ('user');
