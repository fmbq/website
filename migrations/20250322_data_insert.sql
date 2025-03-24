-- Add non-person for FK of noone
INSERT INTO person VALUES(0,"NOT SET","","NOT SET","","","","","","","","","","","");

-- Create person types
INSERT INTO person_type VALUES(0,"QUIZZER");
INSERT INTO person_type VALUES(1,"QUIZMASTER");
INSERT INTO person_type VALUES(2,"COACH");
INSERT INTO person_type VALUES(3,"DIRECTOR");
INSERT INTO person_type VALUES(4,"REGIONAL DIRECTOR");
INSERT INTO person_type VALUES(5,"CONFERENCE DIRECTOR");
INSERT INTO person_type VALUES(6,"GUEST");

-- Add conferences
INSERT INTO conference VALUES(0,"Pacific Northwest");
INSERT INTO conference VALUES(1,"Oregon");
INSERT INTO conference VALUES(2,"The Network of Undeniable Blessing");
INSERT INTO conference VALUES(3,"Pacific Coast Japanese");
INSERT INTO conference VALUES(4,"Southern Califonrnia");
INSERT INTO conference VALUES(5,"The River");
INSERT INTO conference VALUES(6,"North Central");
INSERT INTO conference VALUES(7,"Central Region");
INSERT INTO conference VALUES(8,"Gateway");
INSERT INTO conference VALUES(9,"Wabash");
INSERT INTO conference VALUES(10,"New South");
INSERT INTO conference VALUES(11,"Southeast Region");
INSERT INTO conference VALUES(12,"North Michigan");
INSERT INTO conference VALUES(13,"East Michigan");
INSERT INTO conference VALUES(14,"Southern Michigan");
INSERT INTO conference VALUES(15,"Ohio");
INSERT INTO conference VALUES(16,"The Harvest");
INSERT INTO conference VALUES(17,"Acts 12:24");
INSERT INTO conference VALUES(18,"Keystone");
INSERT INTO conference VALUES(19,"Genesis");
INSERT INTO conference VALUES(999,"Other");

-- Add churches (id,conf_id,title,name,addr,city,st,zip,conf_id);
INSERT INTO church VALUES (0,999,"ONLINE","ONLINE","ONLINE","","","","");
INSERT INTO church VALUES (1,6,"Compass Church","Janesville","1909 Highland Ave","Janesville","WI","53546","compassjvl.com");
INSERT INTO church VALUES (2,8,"Emmanuel Free Methodist Church","Alton","3993 Fosterburg Rd","Alton","IL","62002","efmc.org");
INSERT INTO church VALUES (3,8,"Calvary Baptist Church","Hillsboro","1001 Rountree St","Hillsboro","IL","62049","hillsborocalvarybaptist.com");
INSERT INTO church VALUES (4,9,"Bloomington Free Methodist Church","Bloomington","1121 S Lincoln St","Bloomington","IN","47401","bfmchurch.org");
INSERT INTO church VALUES (5,9,"Bedford Free Methodist Church","Bedford","630 R Street","Bedford","IN","47421","bedfordfreemethodist.org");
INSERT INTO church VALUES (6,9,"Aldersgate Free Methodist Church","Indy-Aldersgate","9035 E 21st Street","Indianapolis","IN","46229","aldersgatefmc.org");
INSERT INTO church VALUES (7,9,"Classical Studies of Indianapolis","CSIndy","2986 Moller Rd","Speedway","IN","46224","csindy.org");
INSERT INTO church VALUES (8,8,"Greenville Free Methodist Church","Greenville","1367 IL-140","Greenville","IL","62246","greenvillefmc.org");
INSERT INTO church VALUES (9,9,"West Morris Free Methodist Church","WEMO","2302 W Morris St","Indianapolis","IN","46221","westmorrisfm.org");
INSERT INTO church VALUES (10,6,"Redemption Center Church","Peoria","2121 N Sheridan Rd","Peoria","IL","61604","redemptionforyou.com");
INSERT INTO church VALUES (11,9,"Vincennes Free Methodist Church","Vincennes","1423 N 4th St","Vincennes","IN","47591","www.facebook.com/VincennesFMC");
INSERT INTO church VALUES (12,10,"Wilmore Free Methodist Church","Wilmore","1200 Lexington Rd","Wilmore","KY","40390","wilmorefmc.org");
INSERT INTO church VALUES (13,19,"New Hope Free Methodist Church","New Hope","62 N Union St","Rochester","NY","14607","newhopefree.org");
INSERT INTO church VALUES (14,19,"Albion Free Methodist Church","Albion","25 S Platt St","Albion","NY","14411","albionfreemethodist.org");
INSERT INTO church VALUES (15,19,"Pearce Church","Pearce","4322 Buffalo Rd","North Chili","NY","14514","www.pearcechurch.org");
INSERT INTO church VALUES (16,5,"Free Methodist Church-Glendale","Phoenix","6331 W Lamar Rd","Glendale","AZ","85301","");



INSERT INTO season_role VALUES (0,"QUIZZER GRADE 1");
INSERT INTO season_role VALUES (1,"QUIZZER GRADE 2");
INSERT INTO season_role VALUES (2,"QUIZZER GRADE 3");
INSERT INTO season_role VALUES (3,"QUIZZER GRADE 4");
INSERT INTO season_role VALUES (4,"QUIZZER GRADE 5");
INSERT INTO season_role VALUES (5,"QUIZZER GRADE 6");
INSERT INTO season_role VALUES (6,"QUIZZER GRADE 7");
INSERT INTO season_role VALUES (7,"QUIZZER GRADE 8");
INSERT INTO season_role VALUES (8,"QUIZZER GRADE 9");
INSERT INTO season_role VALUES (9,"QUIZZER GRADE 10");
INSERT INTO season_role VALUES (10,"QUIZZER GRADE 11");
INSERT INTO season_role VALUES (11,"QUIZZER GRADE 12");
INSERT INTO season_role VALUES (12,"COACH");
INSERT INTO season_role VALUES (13,"QUIZMASTER");
INSERT INTO season_role VALUES (99,"OTHER");



INSERT INTO division VALUES (0,"Newcomer",0);
INSERT INTO division VALUES (1,"JR",1);
INSERT INTO division VALUES (2,"Alpha",2);
INSERT INTO division VALUES (3,"Omega",3);
INSERT INTO division VALUES (4,"Rookie",4);
INSERT INTO division VALUES (5,"MRA",4);
INSERT INTO division VALUES (6,"MRB",4);
INSERT INTO division VALUES (7,"YTR",4);
INSERT INTO division VALUES (8,"YTRA",4);
INSERT INTO division VALUES (9,"YTRB",4);
INSERT INTO division VALUES (10,"STR",4);
INSERT INTO division VALUES (11,"STRA",4);
INSERT INTO division VALUES (12,"STRB",4);
INSERT INTO division VALUES (13,"Veteran",5);
INSERT INTO division VALUES (14,"YTV",6);
INSERT INTO division VALUES (15,"YTVA",6);
INSERT INTO division VALUES (16,"YTVB",6);
INSERT INTO division VALUES (17,"STV",7);
INSERT INTO division VALUES (18,"STVA",7);
INSERT INTO division VALUES (19,"STVB",7);
INSERT INTO division VALUES (999,"NA",999); -- adults and others that don't need a division
INSERT INTO division VALUES (9999,"ALL",9999); -- for quizmasters that don't care what division





-- Add directors into person table (id, firstname, middle, lastname, sex, address1, address2, city, state, zip, phone, email, birthdate, food_allergy);
INSERT INTO person VALUES(1,"Laura","Christensen","Colberg","Laura","F","17750 33rd Avenue NE","","Lake Forest Park","WA","98155","206-218-6575","lchristn12@gmail.com","","");
INSERT INTO person_person_type VALUES(1,3); -- director
INSERT INTO person_person_type VALUES(1,1); -- quizmaster

INSERT INTO person VALUES(2,"Paul","","Stackhouse","Paul","M","","","Wilmore","KY","40390","859-534-7344","pmstackhouse@gmail.com","","");
INSERT INTO person_person_type VALUES(2,4); -- regional director
INSERT INTO person_person_type VALUES(2,1); -- quizmaster
INSERT INTO person_person_type VALUES(2,2); -- coach

INSERT INTO person VALUES(3,"Matthew","David","Coakley","Matt","M","4524 Sandhill Dr","","Janesville","WI","53546","708-334-3234","mattcoakley92@gmail.com","03-30-1974","");
INSERT INTO person_person_type VALUES(3,5); -- conf director
INSERT INTO person_person_type VALUES(3,1); -- quizmaster
INSERT INTO person_person_type VALUES(3,2); -- coach

INSERT INTO person VALUES(4,"Todd","","Stendeback","Todd","M","","","Alton","IL","","","","","");
INSERT INTO person_person_type VALUES(4,5); -- conf director
INSERT INTO person_person_type VALUES(4,1); -- quizmaster
INSERT INTO person_person_type VALUES(4,2); -- coach

-- Janesville
INSERT INTO person VALUES(5,"Stephen","Michael","Coakley","Stephen","M","1455 Walnut Ave","","Janesville","WI","53546","608-371-7841","me@stephencoakley.com","12-12-1994","");
INSERT INTO person_person_type VALUES(5,1); -- quizmaster
INSERT INTO person_person_type VALUES(5,2); -- coach

INSERT INTO person(firstname, middle, lastname, common_name, sex, address1, city, state, zip, phone, email, birthdate)
            VALUES("Michelle","Marie","Coakley","Michelle","F","4524 Sandhill Dr","Janesville","WI","53546","608-314-8840","7disneyfans@gmail.com","10-14-1974");
INSERT INTO person_person_type VALUES(last_insert_rowid(),2); -- coach

INSERT INTO person(firstname, middle, lastname, common_name, sex, address1, city, state, zip, phone, email, birthdate)
            VALUES("Harmony","Noelle","Coakley","Harmony","F","4524 Sandhill Dr","Janesville","WI","53546","608-815-0136","legofriend70108@gmail.com","07-01-2008");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Payton","Alf","Payton","M","2841 Mineral Point Ave","Janesville","WI","53548");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Titus","Alf","Titus","M","2841 Mineral Point Ave","Janesville","WI","53548");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Autumn","Alf","Autumn","F","2841 Mineral Point Ave","Janesville","WI","53548");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Elliott","Brost","Elliott","M","270 Haugen Rd","Edgerton","WI","53534");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Charlotte","Hellmich","Charlotte","F","2931 Mackintosh Dr","Janesville","WI","53548");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Norah","Hellmich","Norah","F","2931 Mackintosh Dr","Janesville","WI","53548");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Adler","Williams","Adler","M","4401 E M H Townline Rd","Milton","WI","53563");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Elijah","Williams","Eli","M","4401 E M H Townline Rd","Milton","WI","53563");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Hazel","Williams","Hazel","F","4401 E M H Townline Rd","Milton","WI","53563");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

INSERT INTO person(firstname, lastname, common_name, sex, address1, city, state, zip)
            VALUES("Zoey","Van Laanen","Zoey","F","745 Chelsea Pl","Janesville","WI","53546");
INSERT INTO person_person_type VALUES(last_insert_rowid(),0); -- quizzer

