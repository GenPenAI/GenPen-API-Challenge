--
-- API Inspector.
-- Prepared SQL queries for 'User' definition.
--


--
-- SELECT template for table `user`
--
SELECT `id`, `username`, `first_name`, `last_name`, `email`, `password`, `phone`, `user_status` FROM `user` WHERE 1;

--
-- INSERT template for table `user`
--
INSERT INTO `user`(`id`, `username`, `first_name`, `last_name`, `email`, `password`, `phone`, `user_status`) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

--
-- UPDATE template for table `user`
--
UPDATE `user` SET `id` = ?, `username` = ?, `first_name` = ?, `last_name` = ?, `email` = ?, `password` = ?, `phone` = ?, `user_status` = ? WHERE 1;

--
-- DELETE template for table `user`
--
DELETE FROM `user` WHERE 0;

