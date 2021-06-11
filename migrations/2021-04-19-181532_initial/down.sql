-- This file should undo anything in `up.sql`
DROP TABLE Customer                 CASCADE;
DROP TABLE Seller                   CASCADE;
DROP TABLE Events                   CASCADE;
DROP TABLE Paintings                CASCADE;
DROP TABLE PaymentStatus            CASCADE;
DROP TABLE PaintingsToPayments      CASCADE;
