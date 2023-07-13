--
-- API Inspector.
-- Prepared SQL queries for 'Error' definition.
--


--
-- SELECT template for table `error`
--
SELECT `code`, `message` FROM `error` WHERE 1;

--
-- INSERT template for table `error`
--
INSERT INTO `error`(`code`, `message`) VALUES (?, ?);

--
-- UPDATE template for table `error`
--
UPDATE `error` SET `code` = ?, `message` = ? WHERE 1;

--
-- DELETE template for table `error`
--
DELETE FROM `error` WHERE 0;

