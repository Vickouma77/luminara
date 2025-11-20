-- Initialize databases for each service

CREATE DATABASE auth_service;
CREATE DATABASE user_service;
CREATE DATABASE account_service;
CREATE DATABASE transaction_service;
CREATE DATABASE kyc_service;
CREATE DATABASE notification_service;
CREATE DATABASE payment_service;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE auth_service TO luminara;
GRANT ALL PRIVILEGES ON DATABASE user_service TO luminara;
GRANT ALL PRIVILEGES ON DATABASE account_service TO luminara;
GRANT ALL PRIVILEGES ON DATABASE transaction_service TO luminara;
GRANT ALL PRIVILEGES ON DATABASE kyc_service TO luminara;
GRANT ALL PRIVILEGES ON DATABASE notification_service TO luminara;
GRANT ALL PRIVILEGES ON DATABASE payment_service TO luminara;
