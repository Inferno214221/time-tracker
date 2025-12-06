-- Your SQL goes here
CREATE TABLE project (
    proj_key        VARCHAR(5) PRIMARY KEY          NOT NULL,
    proj_name       VARCHAR(25)                     NOT NULL
);

CREATE TABLE ticket (
    proj_key        VARCHAR(10) REFERENCES project  NOT NULL,
    tick_num        INTEGER                         NOT NULL,
    PRIMARY KEY (proj_key, tick_num)
);

CREATE TABLE ticket_time (
    proj_key        VARCHAR(10) REFERENCES project  NOT NULL,
    tick_num        INTEGER                         NOT NULL,
    time_id         INTEGER REFERENCES time         NOT NULL,
    FOREIGN KEY (proj_key, tick_num) REFERENCES ticket,
    PRIMARY KEY (proj_key, tick_num, time_id)
);

CREATE TABLE time (
    time_id         INTEGER PRIMARY KEY             NOT NULL,
    time_start      DATETIME                        NOT NULL,
    time_end        DATETIME                        NOT NULL,
    time_desc       VARCHAR(255)                    NOT NULL,
    time_dur        DECIMAL(3,1) GENERATED ALWAYS AS (
        ROUND((JULIANDAY(time_end) - JULIANDAY(time_start)) * 24, 1)
    ) VIRTUAL,
    act_num         INTEGER REFERENCES invoice_activity,
    CHECK (time_end > time_start)
);

CREATE TABLE recipient (
    recip_id        VARCHAR(5) PRIMARY KEY          NOT NULL,
    recip_name      VARCHAR(25)                     NOT NULL,
    recip_addr      VARCHAR(255)                    NOT NULL
);

CREATE TABLE invoice (
    inv_num         INTEGER PRIMARY KEY             NOT NULL,
    inv_month       DATE                            NOT NULL,
    recip_id        VARCHAR(5) REFERENCES recipient NOT NULL,
    inv_created     DATE
);

CREATE TABLE invoice_activity (
    act_num         INTEGER PRIMARY KEY             NOT NULL,
    inv_num         INTEGER REFERENCES invoice      NOT NULL,
    act_desc        VARCHAR(255)                    NOT NULL,
    act_uprice      DECIMAL(6,2)                    NOT NULL
);