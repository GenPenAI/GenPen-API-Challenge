/* SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO"; */
/* SET AUTOCOMMIT = 0; */
/* START TRANSACTION; */
/* SET time_zone = "+00:00"; */

-- --------------------------------------------------------

--
-- Table structure for table `error` generated from model 'Error'
--

CREATE TABLE IF NOT EXISTS `error` (
  `code` INT NOT NULL,
  `message` TEXT NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Original model name - Error.';

--
-- Table structure for table `user` generated from model 'User'
--

CREATE TABLE IF NOT EXISTS `user` (
  `id` BIGINT DEFAULT NULL,
  `username` TEXT DEFAULT NULL,
  `first_name` TEXT DEFAULT NULL COMMENT 'Original param name - firstName.',
  `last_name` TEXT DEFAULT NULL COMMENT 'Original param name - lastName.',
  `email` TEXT DEFAULT NULL,
  `password` TEXT DEFAULT NULL,
  `phone` TEXT DEFAULT NULL,
  `user_status` INT DEFAULT NULL COMMENT 'User Status. Original param name - userStatus.'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Original model name - User.';


