use crate::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Rules",
        html! {
            h1 { "Rules (2021)" }
 
            h2 { "FREE METHODIST BIBLE QUIZZING:"}
            h3 { "Official Rules & Guidelines"}

            p { "2021 Edition @ Free Methodist Bible Quizzing" }
            br { a href="http://www.fmquizzing.org" {"fmquizzing.org"} " | " a href="mailto:fmquizzing@gmail.com" {"fmquizzing@gmail.com"} }
            br { "Effective 9/1/2021" }
            p {" "}
            
            
            hr {" "}
            p { "This Rulebook supersedes all previous Rulebooks.  Participation in Free Methodist Bible Quizzing (\"FMBQ\") 
                constitutes acceptance of these rules and guidelines and any other directives of FMBQ."}
            p { "Free Methodist Bible Quizzing is a ministry affiliated with the Free Methodist Church, USA."}
            p { "The mission of FMBQ is to instill biblical truths in the hearts of teens, establish them in their faith, 
                and prepare them for Christian service through the systematic study, meaningful application, and memorization 
                of the Word of God."}
            p { "Its goal is to encourage young people to regularly study and apply God\'s word to their lives.  The genius of the 
                FMBQ ministry is in the combination of Bible study and competitive sport. The rigors of in-depth Bible study and 
                team practice, the thrill of having memorized Scripture at one\'s mental fingertips, the challenge of the game situation, 
                along with the excitement of quiz trips and meeting new people serve as a magnetic attraction to teens of all ages."}
            p { "The Free Methodist Church was one of the forerunners in using Bible Quizzing as a ministry to young people, 
                and countless lives have been transformed through this ministry.  In many churches, Quizzing has been found to be a 
                valuable part of the overall ministry to youth and evangelistic outreach."}
            
            p { "For information about starting a Bible Quiz team in your church, youth group, home school group, 
                or school, please find more information at " 
                a href="www.fmquizzing.org" {"www.fmquizzing.org"} 
                " or " 
                a href="https://ezcard.com/fmbq620" {"https://ezcard.com/fmbq620"}
            }  

            //***************************************************************************
            //***************************************************************************
            //***************************************************************************
            h2 { "1. GENERAL INFORMATION" }
            h3 { "1.1 ELIGIBILITY FOR PARTICIPATION" }
            h4 { "1.1.0.1 Quizzers and participating churches and teams are not required to be members of the Free Methodist Church." }
            h4 { "1.1.0.2 Individuals finishing grades 6 through 12 are eligible to quiz at Quiz Finals.  " }
            h5 { "(a) Quizzers who will reach the age of 11 before September 1 are permitted to participate during that season, even if they have not yet entered the 6th grade.  Therefore, eligibility begins when students reach the age of 11 by September 1st or enter 6th grade, whichever occurs first.  " }
            h5 { "(b) Quizzers who will reach the age of 19 before September 1, but have not yet graduated from high school are permitted to quiz until the end of that quiz season. In special cases (i.e., remaining in high school after age 19 due to illness or disability), quizzers may petition the denominational leadership for additional eligibility.  Therefore, eligibility ends when a student graduates from high school or reaches the age of 20, whichever occurs first.  This does not prohibit students who are taking college courses from participation, as long as they have not yet graduated from high school. " }
            h5 { "(c) Conferences and regions are encouraged to set up various divisions of competition according to the needs of their area and desires of their coaches.  This may include allowing quizzers younger than 6th grade to compete, but should never prevent quizzers otherwise eligible from participating.  While exceptions may be made by conference tournament directors during the regular season, youth under the age of 11 who are not yet in 6th grade will not be allowed to compete at Denominational Quiz Finals." }
            h5 { "(d) The quiz season is from September 1 through August 31." }

            //***************************************************************************
            h3 { "1.2 SUMMARY OF THE GAME" }
            h4 { "1.2.1 In Free Methodist Bible Quizzing competition, two teams quiz against each other. Each team has two or three quizzers active at any given time, with substitutions permitted. "}
            h4 { "1.2.2 Quizzes are fifteen questions in length, and twenty points are given for each correct response. "}
            h4 { "1.2.3 No points are deducted for incorrect answers in team competition. "}
            h4 { "1.2.4 Only the first four quizzers off their seats are eligible to attempt an answer for any question, with points being given only for the first correct response."}
            h4 { "1.2.5 A single quizzer may correctly answer only 5 questions per quiz.  "}
            h4 { "1.2.6 If a quizzer jumps before the question has been completed and does not correctly complete and answer the question, the question is re-read for the opposing team."}
            h4 { "1.2.7 The winning team is the one with the highest points at the end of the quiz."}

            //***************************************************************************
            h3 { "1.3 TEXT FOR STUDY AND COMPETITION" }
            h4 { "1.3.1 The FMBQ study text may be one book, a combination of several books, or selected passages and may be from either the Old or New Testaments." }
            h5 { "1.3.1.1 All questions and answers used in FMBQ events are from the New International Version (c. 2011)." }
            h5 { "1.3.1.2 The text is normally studied from August/September of one year and culminates with Quiz Finals in June or July of the following year." }
            h5 { "1.3.1.3 The text for the next year\'s competition is usually announced in March or April of the preceding season." }
            h5 { "1.3.1.4 To find out what materials are available for ordering, visit the FM Bible Quiz web site at www.fmquizzing.org and click on \"Study Supplies.\" Some resources (such as the Rulebook) can be downloaded for free from another page at the website, \"Brochure/Rules.\"  Any other questions about resources may be directed to the FM Bible Quiz Director." }


            //***************************************************************************
            h3 { "1.4 OPPORTUNITIES TO PARTICIPATE" }
            h4 { "1.4.1	Monthly.  Local quiz tournaments are held in various parts of the U.S. on a monthly basis, October through April/May.  Each month\'s quiz focuses on the new material studied, with a comprehensive quiz at the end of the season.  Quiz Finals, a weeklong event in the summer, is open to all. Where there is need or demand, an online opportunity to compete will be offered each month, if reasonably feasible to do so." }
            h4 { "1.4.2	Official tournaments. Tournaments sponsored by a Conference or Regional Bible Quiz Director shall be considered \"official\" tournaments, and are entitled to use Official Tournament Questions available from the Denominational Director. " }
            h4 { "1.4.3	Competition styles. " }
            h5 { "1.4.3.1 Round Robin tournaments are those in which each team competes against all other participating teams (or as many as possible within the limitations of the schedule). In this type of competition, the team with the best win/loss record takes 1st place; the team with the second-best record takes 2nd place; etc." }
            h5 { "1.4.3.2 Double Elimination tournaments start by having each team play opponents, with winners proceeding to one bracket and losers to another bracket. Teams are eliminated from the competition with two losses, with the first loss placing the team in the losers\' bracket. Unless defeated twice by the winner of the losers\' bracket, the winner of the winners\' bracket takes 1st place; the winner of the losers\' bracket 2nd place; their final opponent 3rd place; and so on. (For more information on tournament types and how to set up, refer to the Bible Quizzing Handbook available as a free download from the Bible Quizzing web site.)" }
            h5 { "1.4.3.3 Combination Tournaments are those which start with a Round Robin tournament to \"seed\" teams and end up with a double elimination tournament (where teams with the best records are placed so that they will not have to play each other until the final rounds) to decide winners. When time is available to run them, combination tournaments are popular because of greater team participation (Round Robin) and accurate determination of winners (Double Elimination)." }
            h5 { "1.4.3.4 Round Robin Split are those which start with multiple, smaller, round robin tournaments with few enough teams per sub-division to allow that part of the tournament to complete in time for a final one, or two, rounds of elimination tournament play to determine trophy positions.  This tournament is used when there are too many teams for a full round robin in the time available.  Care must be taken to carefully seed the subdivisions so that the higher ranked teams are spread between each initial subdivision." }

            //***************************************************************************
            h3 { "1.5 TEAM CONSTRUCTION" }
            h4 { "1.5.1	Minimum.  A minimum of two quizzers is required for a team to compete in any level of team competition." }
            h4 { "1.5.2	Maximum.  Three contestants typically compose a local team, though a quiz team may enter competition with two quizzers. Up to two substitutes are allowed (five team members total).  Quizzers may not switch to another team after a tournament has started. However, churches may realign their team rosters between tournaments." }
            h4 { "1.5.3	Grade Divisions.  Young Teen teams consist of only teens in grades 6 through 9. Senior Teen teams may consist of quizzers in grades 6 through 12, provided at least one quizzer is in grade 10, 11, or 12.  It is this last condition that requires a team to compete in the Senior Team division. A young teen team with a 9th grader may quiz in either Young Teen or Senior Teen competition." }
            h4 { "1.5.4	Categories of competition.  At both grade levels of competition, there are two types of quizzing: Team Competition and Individual Competition. " }
            h4 { "1.5.5	Competition Divisions.  In tournaments with a sufficient number of teams or quizzers, \"Rookie\" categories (consisting of teams with less experience) may be created at both levels of team and individual competition. A Rookie team consists of quizzers of whom no more than one has quizzed more than half a season prior to the present season.  If there are a sufficient number of Rookie teams, the tournament may divide by grade (i.e., Senior Teen Rookie and Young Teen Rookie).  Where number and demand is appropriate, each division listed in 1.5.5.2 below may be further subdivided into \"A\" and \"B\" divisions, \"A\" denoting stronger competition than \"B\"." }
            h5 { "1.5.5.1 Veteran and Rookie defined.  For purposes of distinguishing between a \"Rookie\" and \"Veteran\" quizzer, a \"Rookie\" has quizzed for five months or fewer in a prior season, and a \"Veteran\" has quizzed more than five months in a prior season, regardless of start/end dates within that prior season." }
            h5 { "1.5.5.2 Possible divisions.  Thus, provision is made for these categories:
                * Young Teen Rookie Team 
                * Senior Teen Rookie Team 
                * Mixed Rookie Team
                * Young Teen Veteran Team 
                * Senior Teen Veteran Team 
                * Young Teen Rookie Individual 
                * Senior Teen Rookie Individual 
                * Young Teen Veteran Individual 
                * Senior Teen Veteran Individual " }
            h4 { "1.5.6	Combined Teams. If at any time during the year two or more churches are unable to form a complete quiz team and wish to combine their quizzers to form a team to compete in conference or regional tournaments, those churches involved must seek permission to do so from either their Conference or Regional Director. Valid reasons (determined by the director) must be offered in order for permission to be granted. " }
            h5 { "1.5.6.1 In order for such a combined team to compete in the Denominational Finals, permission must be obtained from the Denominational Director. Such a request must be submitted to the Denominational Director and must include a recommendation from the team's Conference and/or Regional Directors. This request must be submitted no later than April 1.  Unless prior approval is obtained, a combined team will only be allowed to quiz in a \"B\" division.  Exceptions will be considered on a case-by-case basis." }
            h5 { "1.5.6.2 The request should state the terms and the rationale for the merger. If the Denominational Director deems the request worthy of consideration, they will ask the Regional Directors to consider the merger.  Final decisions on mergers will be made by the Denominational Director in consultation with the Regional Directors. " }
            h4 { "1.5.7	Pooling of Quizzers into Teams. If only one quizzer from a church remains active at the end of a quiz season and that quizzer wishes to participate in the Quiz Finals in team competition as well as in individual competition, he/she may request that his/her name be placed in a \"pool\". A special effort will be made to form teams out of this pool.  A team of \"pooled\" quizzers will only be allowed to quiz in a \"B\" division tournament (following Round Robin rounds) at denominational Bible Quiz Finals." }
            h4 { "1.5.8	Single Quizzer. If a church or school has a single quizzer and is unable to form a legitimate quiz team (2 or more quizzers, regardless of age or experience level) by December 31, the single quizzer may petition the Denominational Director to join the team of a church or school in closest geographical proximity to them.  Such a request must be submitted in writing to the Denominational Director no later than January 15. The request must include a recommendation from the quizzer's Conference and Regional Directors and must be acceptable to both churches or schools involved.  If such a merger is approved, this team may be allowed to quiz in an \"A\" division at Quiz Finals." }

            //***************************************************************************
            h3 { "1.6 ELECTRONIC QUIZ EQUIPMENT" }
            h4 { "1.6.1	A set of quiz lights consists of six seat pads connected to an electronic device that will indicate the order of participants\' jumps to the fourth (or more) place.  A sound shall notify the Quizmaster of the first jump.  Positions should \"lock in\" until all quizzers have returned to a sitting position.  Online competition tools may be approved.  As of this edition, the Buzzer App (www.biblequizshop.net) coupled with online videoconferencing products can serve as a jumping/buzzing option for teams who participate online." }
            h4 { "1.6.2	Hand-activated pressure pads may be substituted when a quizzer is unable to jump because of physical limitations (whether temporary or permanent) and to the extent modified devices are available for this purpose, they shall be used.  A quizzer may also, if feasible, activate the seat pad with his/her hand if a special device is not available." }
        
            //***************************************************************************
            h3 { "1.7 VIDEO RECORDING/STREAMING" }
            h4 { "1.7.1	Video recording and livestreaming of quizzes is permitted as long as it is not disruptive.  Where a quiz is expected to be livestreamed, the participants should be given reasonable notice.  The online address for the recording or stream should be announced to the audience." }
            h4 { "1.7.2	FMBQ retains the rights to all footage containing FMBQ Tournament questions supplied by FMBQ and reserves the       right to ask anyone who posts the video on the Internet to remove it for any reason." }
            h4 { "1.7.3	Recordings are not to be used to verify the correctness of an answer, or as the basis for an appeal." }

            //***************************************************************************
            h3 { "1.8 AWARDS AT QUIZ FINALS" }
            h4 { "1.8.1	Individual and team awards are presented to the top placing teams and to the top individuals at Quiz Finals." }
            h4 { "1.8.2	Scholarships.  Christian colleges and universities, whether affiliated with the Free Methodist Church, USA, or  not, may offer scholarships to FM Bible Quizzers based on participation, achievement, or both.  Information on scholarships will be published as it become available.  In the event of a dispute about qualification for any particular award, the Denominational Director shall make the final determination and all participants waive the right to any legal recourse or challenge to that decision." }
            h4 { "1.8.3	Other competitions.  From time to time, the FMBQ Leadership Team, and/or the Denominational Director may include alternative ways to allow Quizzers to demonstrate proficiency in their study of Scripture.  Monthly practice and participation in alternate modes of study and competition shall be up to each Tournament Director and announced in advance if offered." }



            //***************************************************************************
            //***************************************************************************
            //***************************************************************************
            h2 { "2. FM BIBLE QUIZZING RULES" }
            h3 { "2.1 TEAM & PERSONNEL ARRANGEMENT" }
            h4 { "2.1.1 Quizzers are seated on chairs with the same surface (preferably a hard, flat surface, not a chair with a cushion), facing the audience, one team on each side, in any order they choose." }
            h4 { "2.1.2 The Quizmaster sits or stands in front of the quizzers at a table or podium with his/her back to the audience, facing the teams." }
            h4 { "2.1.3 A timekeeper may be at the table with the Quizmaster or elsewhere in the room." }
            h4 { "2.1.4 A scorekeeper may be at the table with the Quizmaster or elsewhere in the room." }
            h4 { "2.1.5 Each team\'s coach may be at the table, if space allows, or elsewhere in the room." }
            h4 { "2.1.6 Audience members and substitute quizzers should remain behind the Quizmaster\'s table." }
            h4 { "2.1.7 Answer Consultants.  Having answer consultants is not mandatory. However, the Quizmaster may appoint them either before a quiz or while it is in progress if some additional advice is needed in order to make a correct judgment or decision.  If appointed before a quiz, answer consultants should keep their Bibles open and follow along with the questions and answers to keep track of what is said. Ideally, they should be provided with their own set of questions and answers.

            The number of answer consultants used is strictly up to the quizmaster, and their role is solely to give advice when requested. The Quizmaster makes all final judgments. They should speak softly so they won\'t be overheard, and let the Quizmaster give an assessment of the situation before stating their opinion. " }


            //***************************************************************************
			h3 { "2.2 TEAM CAPTAINS" }
			h4 { "2.2.1 One captain shall be chosen by the coach to represent the interests of the team during the competition. The captain shall choose sides, introduce the team, make appeals, request timeouts, call for substitutions and select teammates to answer in the case of jump ties. " }
			h4 { "2.2.2 If a captain leaves the contest for any reason (most commonly from quizzing-out), the coach should appoint an \"acting captain\" and notify the Quizmaster. While competing, an acting captain may perform all the functions of the regular captain." }

            //***************************************************************************
			h3 { "2.3 QUESTIONS" }
			h4 { "2.3.1 Legal Introduction to the Question.  The Quizmaster is to introduce each question in the following manner:  
			\"Question, Question number ___, Question.\"  
			Any marked question type shall be announced in this introduction:  
			\"Question Number Four is a Two-Part Question.\"  
			If the quizmaster inadvertently states this another way and in doing so prompts a forfeit jump or prejump, a procedural appeal may be made, and the question or a substitute question should be read without penalizing the quizzer. If a Quizmaster misstates the introduction, but completes the question with no prejumps being registered, no appeal will be accepted concerning the error in the introduction." }
			h4 { "2.3.2 Legal Reading of the Question.  The Quizmaster shall read questions loudly enough so that all words can be heard easily by all quizzers. In large rooms, a sound system might be used.  The Quizmaster shall read the questions clearly and correctly so that all words are easily understood. Mistakes of pronunciation are permissible only if the words are still clearly recognizable and if the error does not throw off the timing of the question.  The Quizmaster shall read the questions at a moderate pace. Reading too quickly or too slowly, or with inordinate pauses, may trigger unintended prejumping.  If noise interference or other distractions prevent a question from being clearly heard, the quizmaster will re-read or replace the question." }
			h4 { "2.3.3 Ruling on answer.  Quizmasters are encouraged to have their Bibles open and double-check answer sheets with the actual Scripture text.  This is also helpful if a quizzer's answer goes beyond what is written on the question/answer sheet. On any ruling, the decision of the Quizmaster is final unless appealed by a captain or coach before the next question is started. If an appeal is raised, the decision of the Quizmaster on appeal will be final.  The Quizmaster may, on his or her own initiative, correct a decision he/she realizes to have been incorrect when made.  This should be done as soon as the error is realized, and before the next question is ruled upon." }
			h4 { "2.3.4 Appeal.  If any of the conditions above are thought to be violated, an appeal of the procedure may be given. Although Quizmasters are at liberty to seek the counsel of others, they shall be the sole authority on whether the question should be re-read, replaced, or allowed to stand as read." }
			h4 { "2.3.5 Re-reading the Question/Repeating Words.  Ideally, a question is to be read only once in team competition, and re-read only in the event of a missed prejump. " }
			h4 { "2.3.5.1 Veteran competition.  In the event that excessive time is needed by the Quizmaster to make a ruling, consider an appeal, or deal with some other interruption, the question may be repeated. The Quizmaster may also choose to repeat the question due to noise interference. Repeating should include the last full word and any portion of a partially read word.  Re-reading (with no loss of answer time) might be an accommodation for those with special conditions like ADHD. (See section on Special Accommodations, 2.23)" }
			h4 { "2.3.5.2 Rookie competition.  In Rookie quizzes, the question may be re-read upon request, but this shall be done during the requesting quizzer\'s 20-second answer time.  The first person to jump (whether on the initial reading or on the re-reading due to a missed prejump) will not be granted a repeat of the question." }
			h4 { "2.3.6 Clarification allowed.  If quizzers in a veteran quiz are unsure (perhaps due to accent, pronunciation, a homonym, or a disturbance in the room) of a word or phrase read within the question they may ask the quizmaster for a specific clarification between two words or syllables. (\"Did you say \'fall\' or \'full\'?\")  In the case of a completed question the text in question will be immediately repeated by the quizmaster if one of the quizzer\'s suggested options is correct (but shall answer \"No,\" if neither of the options given by the quizzer is correct).  A request for the Quizmaster to repeat the \"last word\" or \"last syllable\" that was read (as opposed to clarifying between two possible words) should be denied in the case of a prejump and is within the quizmaster\'s discretion in the case of a completed question." }
			h4 { "2.3.7 Calling on quizzers.  The quizmaster must call on a quizzer before the quizzer\'s time begins.  Quizzers should be called on by name." }
			h4 { "2.3.8 Identify a prejump.  The quizmaster must signify a \"prejump\" when calling on a quizzer if the completion of the question, not the answer, needs to be given.  Example:  \"Prejump, please complete the question, John.\"" }
			h4 { "2.3.9 Legal Questions.  Tournament questions will be written using the New International Version (c. 2011) of the Bible.  All questions must be based on the announced book(s) of study for that quiz season. No question may be asked which requires a background of general Bible knowledge or depends on any other outside source for its comprehension.  " }
			h4 { "2.3.10 Quote Questions.  Only verses contained in the Official List of Quotes for that quiz year may be used as tournament Quotes.  If an exact quotation is expected for an answer, the word \"quote\" must be both announced as the question type and included in the question. If the question says \"give,\" \"state,\" or \"indicate,\" it does not call for an exact quotation." }
			h4 { "2.3.11 No Yes/No or True/False questions.  Questions may not be asked which require yes/no, or true/false answers." }
			h4 { "2.3.12 Cross Questions.  If the complete answer to a question would require portions of more than one chapter or book, the question should be labeled \"cross-chapter\" or \"cross-book.\"   If the answer requires portions of different parts of the same chapter (separated by at least one verse or in a different paragraph or section), the designation \"all-chapter\" must be used and all portions that fully answer the question must be given.  It is possible that a question that is not \"cross\" early in the season may become \"cross\" when additional chapters are included in the applicable range of study." }
			h4 { "2.3.13 No references used except for Quotes.  Book names, authors and key words or phrases may be used to limit the scope of an answer, but chapter and verse references may not be used for this purpose unless they are among those listed on the Official List of Quotes." }
			h4 { "2.3.14 Content-based, without interpretation.  All questions should be directed toward what the Bible says, not what it means. Interchanging pronouns with proper names and stating other simple variations of the text are acceptable, but questions involving interpretation beyond the obvious implications of the text are not permitted." }
			h4 { "2.3.15 Complete.  Every question must be complete in itself. It should not depend upon a previous question or answer for any information." }
			h4 { "2.3.16 Clarity.  All questions must be worded so that their meaning is clear." }
			h4 { "2.3.17 Question Types to be Announced.  The Quizmaster must notify quizzers of the \"question type\" in advance of the following types of questions: those with two or more parts; those beginning with a "statement" followed by a question; \"cross-chapter\" which require information from more than one of the current quiz chapters; \"cross-book\" questions which require information from more than one of the current quiz books; \"all-chapter\" which require answers from different parts of a single chapter; and \"Quote\" questions. This notification should be given within the legal introduction described above." }
			h4 { "2.3.18 Two-part questions.  A question has two parts if:
			*  There are two subjects with one verb.
			*  There are two verbs with one subject.
			*  There are two question words.
			*  There are two parts (objects) in a key prepositional phrase.
			*  There is a divided predicate (two objects of the verb).

			Note: If the answer is single and applies to both parts, the words \"both ... and\" may be used to eliminate the two parts. Questions that ask for comparisons or relationships between objects are not two parts because these words connote two parts.

			*  A Quote involving more than one of the stated verse(s) references given on the Official List of Quotes." }
			h4 { "2.3.19 Three-part questions.  A question has three parts if:
			* There is a two-part question followed by a one part question.
			* There are three question words in the question with single subjects and objects.
			*  There are three subjects with one verb.
			*  There are three direct objects with one subject and one verb.
			*  There is one subject with three verbs." }
			h4 { "2.3.20 Four-part questions.  A question has four parts if:
			*  It has four question words.
			*  It has a two-part subject and a two-part verb." }
			h4 { "2.3.21 Five- and Six-part questions.  A question with additional multiple parts is normally connected to the number of question words it contains." }
			h4 { "2.3.22 A question shall be considered as a \"legal\" tournament question only if it meets the preceding requirements. Any question which does not meet these guidelines may be appealed. " }

            //***************************************************************************
			h3 { "2.4 QUIZ CONSTRUCTION" }
			h4 { "2.4.1 The following guidelines are suggested (but not required) of those writing tournament questions." }
			h4 { "2.4.2 Progression of Difficulty.  Early in the year the questions will be generally less complex and deal with fewer chapters. Later the questions will become more comprehensive, sometimes asking for facts from several verses, chapters, or even different quiz books. The difficulty of questions will vary for the different divisions (i.e., an \"A\"-level question for Senior Teens or veterans and a \"B\"-level question for Rookie or B divisions)." }
			h4 { "2.4.3 Non-sequential.  Questions should be mixed in sequence; that is, they should not follow verse by verse from the book." }
			h4 { "2.4.4 Answer and reference included.  For the benefit of the Quizmaster and consultants, answers should be written out with the reference clearly indicated immediately following the question. In questions with multiple answers, they should be listed 1, 2, 3, etc., to aid in tracking." }
			h4 { "2.4.5 Question distribution guidelines:   In a 15-question quiz, the following distribution is recommended:
			*  Two Quote Questions;
			*  Two Cross-Questions; one portion of the answer should come from the current month\'s material;
			* After the first quiz of the season, two review questions (exclusively from prior material studied);
			*  Up to five (5) questions in a 15-question quiz might contain review material (i.e., any combination of review questions, cross-questions and Quote questions).
			*  In a 20-question Quiz, an additional Quote, Review and Cross-Question can be added." }
			h4 { "2.4.6 No appeal.  These guidelines are intended to assist Question Writers and to provide quizzers a general expectation within any given quiz round.  Deviations from these guidelines are allowed, so as to avoid too much predictability, and shall not be grounds for an appeal." }

            //***************************************************************************
			h3 { "2.5 POINTS AND SCORING" }
			h4 { "2.5.1 Points for Correct Answer.  Each correct answer, regardless of type of question or difficulty, is awarded 20 points.  In a 15-question quiz, a total of 300 points is available. " }
			h4 { "2.5.2 Quiz-out.  In both Team and Individual Competition, a single quizzer will quiz-out after their individual point total reaches 100 or more points; and that quizzer is not allowed to re-enter that quiz, even in overtime. In Team Competition, a substitute may be appointed to replace the quizzer who has quizzed-out." }
			h4 { "2.5.3 Scorekeeper.  In official quizzes, one scorekeeper will be appointed by the Quizmaster who will record the results of each question on an Official Scoresheet. (See sample scoresheet is at the end of this Rulebook.) A second scorekeeper might be recruited for an accuracy check. (Frequently quizmasters or coaches keep a scoresheet for their own use and can function in this capacity.)  Digital scoresheets are available and encouraged." }
			h4 { "2.5.4 Spaces to complete on scoresheet.  As quizzers are introduced, their names should be recorded in the proper spaces provided on the scoresheet. Also record the team names (and tournament ID numbers - if assigned) at the top of the scoresheet. Warnings and Infractions are recorded with W\'s or I\'s given to the appropriate individual or team by the quizmaster and are recorded next to the specific question where they are issued, as well as at the bottom of the page in the appropriate \"totals\" blanks.  Time-Outs are to be recorded as they are taken by each team." }
			h4 { "2.5.5 Points recorded.  Individual as well as team points will be recorded for each question, with each correct response earning 20 points for both quizzer and team. 20-point deductions (against the team\'s score in team quizzing, but the individual\'s score in individuals) shall be recorded when announced by the quizmaster for a missed appeal. (Not all missed appeals receive deductions.)" }
			h4 { "2.5.6 Scores compiled.  At the end of the quiz, the scorekeeper totals the points for each quizzer and team, enters the totals into spaces provided, and returns the scoresheet to the Quizmaster.  The Quizmaster is responsible for delivering the completed scoresheet to the designated location.  Digital scoresheets shall be sent to the address designated for that tournament." }

            //***************************************************************************
			h3 { "2.6 SCORESHEET INSTRUCTIONS" }
			h4 { "2.6.1  General.  Keeping score for a quiz is a way to be helpful in a quiz tournament. " }
			h4 { "2.6.2  Pen with pressure for paper scoresheets.  Use a ballpoint pen and apply sufficient pressure, especially when using triplicate scoresheets. Before the quiz starts, fill in all blanks at the top of the sheet. Write each quizzer\'s name in the rectangles numbered 1-6. (Note there is space for 2 teams.) Use the quizzer\'s first and last names or at least an initial.  A laptop or electronic tablet or similar device will be needed for digital scoresheets." }
			h4 { "2.6.3  Correct answers.  Each correctly answered question is worth 20 points. Mark a 20 in the column of the quizzer who answered, then add twenty to that team\'s running score and mark it in the middle column." }
			h4 { "2.6.4  Inset box.  The small box inside each of the individual scoring boxes is used for two similar functions:" }
			h4 { "2.6.4.1 Note the prejump.  Where there is a prejump, mark the box with an X in the appropriate quizzer\'s column. If he or she proceeds to answer correctly, 20 points are given. " }
			h4 { "2.6.4.2 Missed prejump completion or answer.  If, on the other hand, the quizzer cannot complete the question correctly, or answers incorrectly, the small box should be darkened in. This is important especially in Individual Competition when penalties are made against the quizzer who accumulates too many.  Using an \"M\" on the digital scoresheet will fill in this box." }
			h4 { "2.6.4.3 Jump order.  The box may be used to record jumping order. If there is no prejump, place a 1 in the small box of the first quizzer to jump, a 2 in the box of the second, all the way to the fourth. This is reflected on the electronic jump seats display. " }
			h4 { "2.6.5 Running score.  The center columns are used for the running score. When a quizzer scores, a \"20\" is placed in his or her personal column, and the running score is adjusted accordingly. Mark these columns promptly, as the Quizmaster may ask you for the official score at any time." }
			h4 { "2.6.6 Individual Competition. The running score columns are not used in individual competition.  Instead, each quizzer\'s running score is maintained in their own column." }
			h4 { "2.6.7  Questions \"in.\"  An important feature of the quiz scoresheet gives the statistician the ability to calculate a weighted average for each quizzer. These are the boxes at the bottom of the scoresheet under the caption \"No. of Questions \'In\' Per Quizzer\". When a quizzer is substituted out of or into a quiz, draw a dark line on the line directly beneath the box of the question that was just completed (in the column of that quizzer). At the end of that quiz, you can quickly determine the number of questions a quizzer was \"in.\" " }
			h4 { "2.6.8 Totals.  At the end of the quiz:
			*  Total all columns, including the final score.  Digital scoresheets will auto-tally.
			*  Put the number of questions that each quizzer was \"in\" the match in the appropriate boxes. (For most quizzers this number will be 15, but if they quizzed out or were substituted in or out, this number will be less.)
			*  Have a representative from each team initial the scoresheet next to the team name.  If digital scoresheets are used, give each captain or coach the opportunity to view the scoresheet onscreen before submitting.
			*  Give the scoresheet to the Quizmaster to be checked, initialed, and distributed. In the case of triplicates, the top page is to be turned in to tournament officials, and the bottom two sheets can be given to coaches.  Digital scoresheets can be emailed to each coach, captain, church representative or others as determined by the tournament director." }

            //***************************************************************************
			h3 { "2.7 QUIZ FORFEITURE" }
			p{ "Failure to appear.  Any team failing to appear within ten minutes after the announced beginning time (except for completely unavoidable circumstances) will automatically \"forfeit\" the round. Acceptable lateness might include a prior quiz running late, a mix-up on time and place where the participating team is not at fault, a transportation breakdown with the late team still arriving within a reasonable period of time, etc. The tournament coordinator (upon notification from the Quizmaster) shall rule on the validity of the team\'s reason for lateness. The decision is final. " }

            //***************************************************************************
			h3 { "2.8 TIME LIMITATIONS" }
			h4 { "2.8.1 Time limits are as follows:
			*  5 seconds ... to stand after a question has been completed by the Quizmaster.  A quizzer who has not stood up within this period of time may not answer and if they are called on to answer, the Quizmaster is to be alerted immediately that the quizzer is ineligible to answer.  In online quizzing, this time period applies to \"buzzing in\" to register an intent to answer.
			*  20 seconds ... to complete a prejump after being recognized by the quizmaster.
			*  20 seconds ... to answer a question, whether it was finished by the quizmaster or completed by a quizzer who prejumped. This time period starts when the quizzer is recognized by name, or told he/she may \"proceed\" or \"answer\" the question he/she completed.
			*  30 seconds ... to present an appeal or to rebut an appeal.
			*  60 seconds ... for Time-outs.
			*  10 minutes ... before a team forfeits a quiz if the Tournament Director (upon notification from the Quizmaster) does not find the team\'s reason for lateness satisfactory (based on the announced or revised starting time for the quiz).  All forfeits must be approved by the Tournament Director." }
			h4 { "2.8.2 Timekeeper.  One timekeeper will be appointed by the Quizmaster to keep track of and report the ending of all time limits described below. The timekeeper might also be instructed to call out \"JUMP\" when any light comes on (not noticed by the Quizmaster) after the third introductory word \"question.\" This means that the timekeeper must be positioned to clearly see all lights." }
			h4 { "2.8.3 Call \"Time.\"  A timekeeper is to announce any end of time limits by loudly and clearly calling out the word, \"TIME.\"  In tournament play it is strongly preferred that a device be used that can automatically signal the end of the time with a tone.  " }
			h4 { "2.8.4 Tie goes to the quizzer.  If the Timekeeper calls \"Time\" (or an electronic tone sounds, if used) as the quizzer is saying his/her very last word of the answer, the benefit will be given to the quizzer and, if the answer is correct, points will be awarded." }

            //***************************************************************************
			h3 { "2.9 JUMPING" }
			h4 { "2.9.1 Legal Jump.  A legal jump is any activation of the electronic quiz box light; providing a legal jumping posture has been adopted, the Quizzer continuously rises from the jump position and does not sit back down.  If a light goes on and off quickly and the quizzer continues to rise, this constitutes a legal jump, even though the light may no longer register due to a malfunction." }
			h4 { "2.9.2 Legal Jumping Position.  All contestants must be seated on chairs facing the quizmaster. The seat pad must be flat on the top surface of the chair. Quizzers must activate their light in a manner that brings them toward a standing position in a smooth motion.  Quizzers may not touch any part of the chair, floor or any object with their hands; feet may only touch the floor.  The use of non-slip material between a seat pad and the chair is allowed." }
			h4 { "2.9.3 Ties in jumping on a completed question.  If two quizzers on opposite teams jump simultaneously, one quizzer shall answer privately to the Quizmaster (outside the room to prevent others from hearing), and the other quizzer will answer before the audience. If both are correct, each team receives the total point value of the question.  The Quizmaster shall not rule on either answer until both are heard.

			If two quizzers on the same team jump simultaneously, the captain shall choose the first to answer to the audience. If declared incorrect, then and only then shall the other quizzer be called upon to answer. In no situation may one team score more than 20 points per question." }
			h4 { "2.9.4 Tie on a Prejump.  If members of opposing teams tie on a prejump, one shall complete the question privately to the Quizmaster and the other aloud to the audience. If only one quizzer\'s completion is acceptable, that quizzer will answer aloud to the audience. If both are correct, one shall answer privately to the quizmaster and the other aloud. If neither are correct, either in completing the question or answering it, the question will be declared closed and the quizmaster will move on to the next question.

			If teammates tie on a prejump, only the quizzer selected by the captain to complete the question will be considered. If incorrect, the question will be re-read for the other side as in a normal prejump." }

            //***************************************************************************
			h3 { "2.10 TIES" }
			h4 { "2.10.1 In a quiz.  In case of a tie at the end of a regulation quiz, there will be an overtime in which three questions will be asked. If there is still no winner, one question will be asked until a correct response determines the contest (\"sudden death\"). Before the overtime begins, a free time-out will be granted and substitutions may be made. No further time-outs will be allowed during the overtime (unless called by the Quizmaster)." }
			h4 { "2.10.2 In a tournament.  If two or more teams tie in win/loss record for a Round Robin Tournament the primary deciding factor will be who won the head-to-head competition (when the teams played each other). In the case of a circular tie where team A beat team B, team B beat team C, and team C beat team A, a point differential method (which compares the scores only from rounds when the three teams played each other) will be used to rank the placement of all three teams  In the rare case where teams are tied in all of these areas, the tournament director may either hold a three-question play-off between the teams or use another equitable method which may include the sum of all points earned by the teams across the day.  This should always be a last resort method to encourage teams to always practice good sportsmanship.  It is likewise acceptable for the tournament director to declare any ranking or award to be a tie. " }

            //***************************************************************************
			h3 { "2.11 SUBSTITUTIONS" }
			h4 { "2.11.1 Defined.  A fourth or fifth team member may be considered as a \"substitute quizzer.\"" }
			h4 { "2.11.2 Limits.  Substitutions may be made by the coach or captain any time between questions, up to a maximum of five times per quiz." }

            //***************************************************************************
			h3 { "2.12 TIME-OUTS AND HALF-TIME" }
			h4 { "2.12.1 Requesting a Time-out.  Two sixty-second time-outs per team in a team quiz, and one sixty-second Time-out per Individual in an Individual quiz, may be requested by coaches or captains or Individuals during a quiz, and granted by the Quizmaster. Time-outs may be called only after a question is closed and before the next question is introduced. As a courtesy to both teams, when an appeal is being considered and when a team asks for a Time-out, the quizmaster is encouraged to ask both teams if they are ready for the question to be closed before allowing the Time-out.  A Time-out is not declared when requested, but rather when the Quizmaster grants it." }
			h4 { "2.12.1.1 Cut-off for requesting a Time-out.  No Time-outs may be called after the beginning of the third question from the end of the quiz; i.e., question 13 in a 15-question round, or question 18 in a 20-question round, nor after the start of Overtime. An official Time-out may be called by the Quizmaster at any time, even when a question is still \"open.\" (The Quizmaster may need this time to check for equipment failure, seek advice, or do anything else necessary for the fair awarding of points on any given question.)" }
			h4 { "2.12.1.2 Access to materials.  Quizzers may refer to their text during a Time-out." }
			h4 { "2.12.1.3 No appeals following Time-out.  No appeals may be raised on issues that preceded a Time-out after a Time-out is granted." }
			h4 { "2.12.1.4 Official Time-out.  If assistance is needed in making a ruling or processing an appeal, the Quizmaster can call an official Time-out and seek guidance from the tournament director or another experienced quizmaster." }
			h4 { "2.12.2 Half-Time.  The quizmaster shall announce \"Half-Time\" after the 8th question of every quiz. No time will be given for a Time-out (unless needed by the quizmaster), but an opportunity will be extended for teams to switch sides if either captain or coach so desires." }

            //***************************************************************************

			h3 { "2.13  ANSWERING" }
			h4 { "2.13.1 Who may answer.  The privilege of answering a question is restricted to the first four quizzers who register on the jump seats, provided that each has met the conditions of a legal jump.  No quizzer failing to jump within 5 seconds after the completion of the question will be allowed to answer. If no quizzer jumps within these 5 seconds, the question will be closed, the answer read, and the next question begun.  Quizzers shall not have the privilege of answering if they sit down on their quiz seat after the activation of their light or end the answer period in any other way, or commit any type of forfeit jump." }
			h4 { "2.13.2 How to answer.  When answering, quizzers should face the Quizmaster and give answers which are loud and clear enough so that the Quizmaster can understand and distinguish words without having to guess at them. If an answer has been mumbled, or the quizzer gives their answer while facing away from the Quizmaster, or far enough away that the answer cannot be clearly heard, it shall be declared incorrect. If it is \"borderline\" in its delivery, the Quizmaster may ask for clarification or repetition, even after the answer period has ended. The first time a "borderline" answer is given, the Quizmaster might ask the quizzer to repeat the answer, and a caution should be given; the next time an answer cannot be clearly understood, it should be counted incorrect without the opportunity to clarify or repeat." }
			h4 { "2.13.3 Answer period.   The answer period shall begin whenever a quizzer is called upon by the Quizmaster. The answer period shall end when one of the following three occur: 1) the quizzer says \"finished\"; 2) the quizzer sits down on the quiz seat; or 3) the time limit (20 seconds) expires. No part of the answer given after the time limit will be considered." }
			h4 { "2.13.4 Pronunciation.  Words which are mispronounced shall be counted correct if they are still recognizable and can be distinguished from all similar words in that season\'s text.  A Quizmaster may ask for a mispronounced word or name to be spelled even after the answer period is closed to be sure of what a quizzer intended." }
			h4 { "2.13.5 An Answer is Correct when . . ." }
			h4 { "2.13.5.1 It gives exactly the information asked for." }
			h4 { "2.13.5.2 Added information is also accurate provided it is not excluded by a legal limiter." }
			h4 { "2.13.5.3 If added information is ambiguous due to a legal \"limiter\" in the written question, the quizzer must include and distinguish the limiter(s) in their answer." }
			h4 { "2.13.5.4 The \"correction\" of an answer is accurate. " }
			h4 { "2.13.5.5 The answer is clearly stated with no words being mispronounced beyond recognition. " }
			h4 { "2.13.5.6 The Scripture containing the complete answer is given accurately." }
			h4 { "2.13.5.7 Quotes are given exactly word-for-word, with no additions or deletions.  
			Note: The exact words of Scripture are required only for \"Quotes.\" A general answer which is correct in meaning and inclusive of all key concepts is sufficient for all other questions." }
			h4 { "2.13.5.8 The answer to a question is ambiguous due to a variety of interpretations of a passage; points are awarded to the first quizzer giving a possible correct answer." }
			h4 { "2.13.6 An Answer is Incorrect when . . ." }
			h4 { "2.13.6.1 It contains inaccurate information for the whole answer or any part thereof.
			Note: The biblical text itself must be consulted when a quizzer\'s answer goes beyond the answer printed on the Question sheet." }
			h4 { "2.13.6.2 It is incomplete." }
			h4 { "2.13.6.3 Incorrect Scripture references are given with the answer when required (Quote questions only)." }
			h4 { "2.13.6.4 An exact quotation is required and even one word is added, omitted, or altered. 
			Note:  This rule disallows answering a Quote question with any additional passage other than the one requested in the question." }
			h4 { "2.13.6.5 Any significant word is mispronounced beyond recognition or in such a way as to be confused with another word of similar pronunciation in the text for that season." }
			h4 { "2.13.6.6 Information excluded by the wording of the question is given." }
			h4 { "2.13.6.7 Words or prepositions are interchanged thus changing the meaning of what the Bible says." }
			h4 { "2.13.6.8 Pronouns are incorrectly defined. If the question asks who did something, then the quizzer must identify, without prompting, who the passage is talking about, even if the verse in question does not specifically mention the name or group.  Quizmasters may have to decide whether other questions require identifying a pronoun, but typically the answer sheet will give guidance.  When pronouns need to be clarified and are not, and if the rest of the answer is correct, the quizmaster is encouraged to ask the quizzer to clarify the unidentified pronoun.  If they are clarified correctly, the answer is correct.  If they incorrectly identify the pronoun, the answer is incorrect." }
			h4 { "2.13.7 Corrections allowed.  Quizzers may correct themselves at any time during the answer period. They must use the word \"correction\" to make any changes in an answer, and must clearly indicate what is to be corrected to the satisfaction of the Quizmaster. If a quizzer says \"correction,\" and time runs out before the correction is made, the answer will be ruled correct if the original answer was correct and if no incorrect information was inserted within the time limit. No corrections made after the time limit will be considered." }
			h4 { "2.13.8 Repeated words.  In answering a Quote question, if a quizzer stops to think and when beginning repeats part of what has already been answered without using the word \"correction\" and it is correct aside from the repetition, the answer shall be ruled correct." }

            //***************************************************************************

			h3 { "2.14 PREJUMPS" }
			h4 { "2.14.1 Defined.  In Team Competition, a \"prejump\" shall be called any time a quizzer\'s light comes on after the first word of the question has been started and before the final word has been completed. (For an exception, see Equipment Malfunction.)  If the final word is simply \"in progress\" when the jump occurs, the Quizmaster shall be the sole judge of whether or not to finish the word or call a prejump. " }
			h4 { "2.14.2 Confirm prejump.  Because a prejump can sometimes be mistaken for a completed question, when recognizing a Quizzer, the Quizmaster must indicate that the question has been prejumped and requires a completion, not an answer.  Stating the word \"prejump\" before the quizzer\'s name is sufficient, or asking the quizzer to finish (versus answer) the question. If this designation of a prejump is not made, an appeal of procedure may be in order if points are placed in jeopardy." }
			h4 { "2.14.3 Completion required.  On a prejump, the contestant must finish the question to the satisfaction of the Quizmaster within 20 seconds, and then (if told to proceed) answer the question within 20 additional seconds. An exact, word-for-word completion is not required, but it must require the same answer as written on the Question page." }
			h4 { "2.14.4 Question re-read if missed.  On a prejump, if either the completed question or answer is incorrectly stated by the prejumping quizzer, all quizzers will take their seats, and the question will be re-read for the opposing team only. Regular rules for jumping, timing, and scoring apply when the question is re-read.  If the same question is pre-jumped while being re-read, only the person who prejumped will be called upon to both complete the question and, if sufficiently completed, to give an answer." }
			h4 { "2.14.5 Completion not required.  Quote questions which contain words or phrases from the Quote, or which ask questions intended to lead to the correct passage, will be treated as follows:  The prejumping quizzer, when called upon, will be asked to give the answer without having to finish the question. If missed, it shall be treated as a missed prejump." }
			h4 { "2.14.6 A Prejump is correct when ..." }
			h4 { "2.14.6.1 The exact question is given." }
			h4 { "2.14.6.2 The general question is given if it meets all other conditions below.
			*  The question word has been substituted with another question word of similar meaning.
			Note: The basic question words are: Who (Whom), What, When, Where, Why, How, For what reason, purpose, or cause. 
			\"For what Reason\" is inter-changeable with \"Why.\" 
			\"Who\" may be interchangeable with \"What\" in some contexts (although it is preferred that \"Who\" and \"What\" be used properly, of course).
			* The prejump is completed correctly and extra accurate information not already a part of the written answer is added, such as references or clarifying modifiers of words to the question.
			* The question has a list for an answer and the completion asks for as many or more parts of the list than the written question.  Completing a question with \"however many parts\" is acceptable." }
			h4 { "2.14.6.3 In ruling on the correctness of a completion of a question in a prejump, the quizmaster should take an approach of allowing quizzers to show what they know. If the completion of the question given by the quizzer would lead to the same answer written in the question, the quizmaster should rule \"you may proceed\" (making no special indication that this is a close situation) and allow the quizzer to attempt to answer the question. The written answer must be contained within the verbal answer. If it is not, even though the answer may be adequate for answering the quizzer's question, the quizzer must be ruled \"incorrect.\" If the Quizmaster rules \"incorrect,\" the question must be re-read as written. The following are guidelines for quizmasters:
			*  The completed question must have the same number of parts as the written question.
			* A significant part of the written answer must not be included in the completed question. 
			* The answer to the written question must be included in the answer to the completed question. If any part of the written answer would not be required by the quizzer\'s completed question, the completion should not be accepted.
			* The answer provided by the quizzer must answer the written question and his/her completed question, even if the completed question is broader and requires more information to be answered correctly, even if that information comes from a different chapter.
			*  The completed question must be specific enough to be reasonably answered within 20 seconds (when in doubt, the Quizmaster is encouraged to allow the Quizzer to try to answer his/her completed question)." }
			h4 { "2.14.7 No change to question words read.  Quizzers cannot change what the Quizmaster has already said in a question that is prejumped.  The question must be completed by using what has already been said by the quizmaster." }
			h4 { "2.14.8 Expansion to cross-question allowed.  It is acceptable for a quizzer to turn a regular question into an all-chapter, cross-chapter, or cross-book question by making it more broad than written.  Then the quizzer must answer the broader question by giving all answers to his/her question. However, the completed question must still have the same number of parts as the written question." }
			h4 { "2.14.9 May ask if the question can be complete.  If a quizzer jumps at such a place in a question where the question could be considered complete but according to the Question page is not complete, when asked to finish the question, the quizzer may say to the Quizmaster, \"My question is complete\" or \"May the question be considered complete as is?\" or something similar.  After the completion period is over, the Quizmaster may accept that as a sufficient completion if grammar and context allow.  Silence is not to be construed as a request to allow the question to be considered complete; this must be stated as the quizzer\'s intention." }
			h4 { "2.14.10 A Prejump is Incorrect when . . ." }
			h4 { "2.14.10.1 The correct question word (or its acceptable equivalent) is not given." }
			h4 { "2.14.10.2 A key part of the answer has been placed in the completion of the question. The Quizmaster must interpret how significant the \"part of the answer\" is.  One or two words (or more) of the answer may be acceptable in the question if it is not a significant part of the answer." }
			h4 { "2.14.10.3 The question has not been given a proper completion, makes no grammatical sense, and/or key words or phrases have been omitted.  If the prejump occurred at the beginning of a word that the quizmaster did not finish, the completion must include that word (or the remainder of it)." }
			h4 { "2.14.10.4 More or fewer parts have been given than were in the original (written) question, such as a 2-part question for a 3-part question, etc." }
			h4 { "2.14.10.5 The question has a list for an answer and the completion asks for fewer parts of the list than the written question." }
			h4 { "2.14.10.6 The completion includes limiters such as:
			* chapters, verses, sentences or paragraphs, such as \"...and what is said in the rest of this paragraph?\" or 
			* an open-ended completion, such as \"... and what else is true?\" when not limited by another part of the question, or
			* when the completion makes use of a construction of otherwise unrelated words in the text to limit the answer to a contiguous passage, such as \"and what else is said between ... and ...\"  or \"and what else is said up to the point . . .\" or other words to the same effect;
			*  The completion of a multi-part question is unrelated to rest of the question;" }
			h4 { "2.14.10.7 The question was announced as a \"statement followed by a question\" type and the quizzer failed to give (or complete) a statement followed by a question." }

            //***************************************************************************
			h3 { "2.15  FORFEIT JUMPS" }
			h4 { "2.15.1 Defined.  In Team Competition, a \"forfeit jump\" shall be called if, after the third introductory word \"question\" has been read by the Quizmaster, one of the following occurs: 
			1) a quizzer\'s light comes on before the first word has been read; 
			2) a light flickers and the quizzer remains seated soon after the question is \"open\"; or 
			3) a quizzer has not yet adopted a legal jumping position at the start of the question following the introduction. 
			" }
			h4 { "2.15.2 Penalty.  The violating quizzer must \"sit out\" that question and forfeit any chance of answering.  Teammates, however, may jump normally and are eligible to answer and earn points for the team. This ruling does not apply if there is an equipment malfunction.  The quizzer may resume jumping on the next question." }
			h4 { "2.15.3 Question re-read for all others.  On a forfeit jump, the Quizmaster shall repeat the introduction to the question so that quizzers may re-position themselves for the re-reading of the question." }
			h4 { "2.15.4 Not a forfeit if lights not reset.  If lights were on prior to the third introductory word \"question\" and continue on through the pause and start of the question, no forfeit jump or prejump shall be called. Rather, the Quizmaster shall start over or read another question. If, however, the lights flicker on after the question is open, it is called a forfeit jump.  In essence, a forfeit jump will be called before the first word is read and a prejump when the first word has been read or partially read." }

            //***************************************************************************
			h3 { "2.16 INDIVIDUAL COMPETITION" }
			h4 { "2.16.1 Eligibility - Quizzers in grades 6-9 quiz in Young Teen Individual Competition, and those in grades 9-12 quiz in Senior Teen Individual Competition. Ninth graders may choose whether to quiz in Young Teen or Senior Teen Individual Competition." }
			h4 { "2.16.2 Length of quizzes.  Individual Competition quizzes will be 15 questions in preliminary rounds, and 20 questions in the final round. Conference and regional tournaments may use 15 questions for the final round of individuals to speed up the quiz day.  The tournament director should make this decision and announce it before Individual competition begins. Procedures for advancing quizzers into the final round and how ties will be broken should also be announced." }
			h4 { "2.16.3 Ranking.  Winners will be determined first by the order in which quizzers quiz-out and then by the highest point totals. Ties after the final question will be decided in a three-question overtime, or by an alternate method announced prior to the competition. If an overtime is needed to break a tie and one of the quizzers reaches 100 points before the 3 questions are asked, that quizzer is the winner of the tie-breaker by quizzing-out before the other.  " }
			h4 { "2.16.4 No forfeit jumps.  Jumps after the third introductory word \"question\" and before the first word of the question are not considered \"forfeit jumps\" in Individual Competition; rather, any jump after the third introductory word \"question\" on up until the final word of the question will be considered a prejump." }
			h4 { "2.16.5 Pass or play.  If a prejumped question is not correctly completed, the next three remaining quizzers who have also jumped will be given a chance to \"play\" by attempting to complete the question, or to \"pass\" on a completion attempt. The Quizmaster shall reread the question up to the place of the initial prejump, and then ask if the next quizzer wants to pass or play.  Quizzers must either sit down or say the word \"pass\" or nothing at all in a 20-second period in order to pass. Their 20-second prejump completion time begins when they are asked if they want to \"pass or play.\" If none of the four quizzers to jump provides a correct completion, the entire question will be reread for everyone except those who incorrectly completed the question. Any missed prejumps (either completion or answer portion) are subject to a prejump penalty. If a prejumped question is completed correctly, but an incorrect answer is given, the question will not be reread; rather, the quizzers still standing will be allowed to answer the prejumped/completed question in the order of the jump until a correct answer is given or no quizzers are left standing, or four attempts at the answer have been made, including the first attempt from the quizzer who prejumped." }
			h4 { "2.16.6 Re-read for discretionary completions.  If a quizzer's completion is accepted, but not the same as the written question, and the quizzer gives an incorrect answer, the quizmaster shall reread the question to the point of the prejump before asking the next quizzer to pass or play.  A response like \"same as his\" is not acceptable.  Each quizzer must complete the question--though a subsequent quizzer could say the exact same words as the prior quizzer who was allowed to answer on the basis of that completion." }
			h4 { "2.16.7 Missed prejump penalty.   After a quizzer in Individual Competition misses three prejumps (either completion or answer portion), the quizzer shall be assessed a 10-point penalty for every missed prejump (completion or answer) thereafter.  In order to quiz-out, a quizzer must have a score of at least 100 points.  As a result of this penalty, a quiz-out score may be 110 points, or a quizzer may need to answer six or more questions (not five)." }
			h4 { "2.16.8 Appeals.  During Individual Competition, only participating quizzers are allowed to appeal. These appeals are made before all quizzers, with a chance for rebuttal. A 20-point penalty will be individually deducted if the appeal is of the type that places points in jeopardy and is overruled.  More than one appeal may be made by different quizzers in an Individual round on the same question or answer or other issue, until the matter is resolved or all appeals are ruled upon." }
			h4 { "2.16.9 Time-outs.  Individual quizzers will be allowed one 60-second Time-out each in any Individual round." }
			h4 { "2.16.10 Seat selection.  Seat placement for Individuals may, at the request of any quizzer or by determination of the Quizmaster, be assigned by having each quizzer draw a piece of paper or other token with a number for each seat available.  Starting with number 1, each quizzer may choose his/her seat.  At the halfway mark (after Question 8 in a 15-question quiz, or after Question 10 in a 20-question quiz), if any quizzer wishes to change seats, all quizzers shall stand and, starting in reverse order (i.e., with the quizzer who drew the last number), each quizzer shall again choose his/her seat for the remainder of the quiz." }

            //***************************************************************************
			h3 { "2.17 APPEALS" }
			h4 { "2.17.1 General intent.  The purpose of any type of appeal is first, to allow for the proper awarding of points according to the rules; and second, to increase the quizzers\' and coaches\' confidence that the rules will be consistently applied by giving them opportunity to call attention to the rules when they feel one has been overlooked or misapplied. The intent of an appeal is not to steal points away from the opposing team on a technicality. The person making the appeal should keep in mind the goal of quizzing is to encourage the youth of both teams in their study of God\'s word." }
			h4 { "2.17.2 Who may appeal.  Only the team captain may make appeals on the question, answer, prejump completion, or the procedure; the coach may only appeal procedure." }
			h4 { "2.17.3 Outcome.  An appeal shall be accepted only if the Quizmaster realizes that the original ruling was evidently in error with the rules. The rules specific to the type of appeal being given should be referred to during deliberations. The Quizmaster should listen carefully to both appeal and rebuttal, consult the rules, and rule fairly and impartially. Warnings and Infractions should not be given lightly or without cause, but neither should they be withheld if one is due." }
			h4 { "2.17.4 Replacement question for all.  A question is discarded and a substitute question read for both sides when . . ." }
			h4 { "2.17.4.1 The question as written has no answer (or none that can be given in 20 seconds)." }
			h4 { "2.17.4.2 The question has been read by the Quizmaster in violation of the legal reading guidelines." }
			h4 { "2.17.4.3 The question has been improperly introduced by the Quizmaster (i.e., it has more or fewer parts, is a different question type than noted by the Quizmaster in the introduction), unless the question was completed by the Quizmaster before anyone jumped." }
			h4 { "2.17.4.4 The question does not follow the stated guidelines for legal questions. " }
			h4 { "2.17.4.5 The quizmaster has added extra pertinent information in re-reading a prejumped question." }
			h4 { "2.17.4.6 An appeal of the prejump is accepted after the question has been answered correctly." }
			h4 { "2.17.4.7 The question requires interpretation." }
			h4 { "2.17.4.8 A prejump has been caused by mechanical malfunction." }
			h4 { "2.17.5 Replacement question for other team.  A substitute question shall be read just for the opposing team when. . ." }
			h4 { "2.17.5.1 One team prejumps and misses the question or answer, and then, prior to reading the question to the opposing team, something occurs that forces the quizmaster to throw out a good question." }
			h4 { "2.17.6 Timing.    An appeal may be given by a captain any time the question is still \"open,\" that is, before the next question has been started or a Time-out has been granted for either team, as long as it does not interrupt a contestant\'s \"answer period.\" " }
			h4 { "2.17.7 Multiple appeals.  Only one appeal of the question will be allowed per question. Only one appeal of the answer or prejump completion will be allowed per answer or completion.  (A captain is given an opportunity for rebuttal on appeals of answers and prejump completions so there is no value in allowing a second appeal should the quizmaster accept the appeal and reverse the ruling.)
			However, an appeal of procedure may be raised by the coach or captain, even for a question which has already drawn another type of appeal if some other issue which jeopardizes the fair awarding of points is at stake.  " }
			h4 { "2.17.8 No access to text.  Bibles or Scripture portions may not be used by a captain during an appeal. The Rulebook may be used by either captain or coach when making an appeal of procedure." }
			h4 { "2.17.9 Rebuttal.  If the captain (or coach) appeals, then the opposing captain (or coach, respectively) is allowed to rebut. " }
			h4 { "2.17.10 Timing.  Thirty seconds shall be allowed for the appeal, and thirty seconds for the rebuttal." }
			h4 { "2.17.11 Points at stake.  It is a courtesy for the Quizmaster to state when the appeal is raised as to whether or not any points may be lost if the appeal is overruled. If the person appealing states that a certain type of appeal is being made, but it becomes evident that another type of appeal is actually being raised, the Quizmaster should inform the person appealing and ask for clarification on the type of appeal being given." }
			h4 { "2.17.12 Clear eligible quizzers.  Except for an appeal of procedure (where all quizzers may remain in the room), quizzers in a team quiz who are still eligible to answer the question will leave the room during the appeal and rebuttal, except for the team captain who is called upon to rebut the appeal." }
			h4 { "2.17.13 Process.  The appeal shall be made aloud to the Quizmaster and audience, with no discussion other than requested by the Quizmaster. The Quizmaster will make no evaluation until after the rebuttal is presented. Even if the Quizmaster does not agree with the presentation of the appeal, the appeal should be accepted if the original ruling was incorrect." }
			h4 { "2.17.14 Rules must be present.  The appealing coach or captain must have a copy of the rules present to make a procedural appeal, and shall have access to them while the appeal is in progress. When possible, the specific rule infraction should be pointed out by the person appealing. The opposing captain or coach does not need a copy of the rules to make a rebuttal, but should be as specific as possible and may use the rules if desired. If the appeal is accepted, the Quizmaster must take whatever action is necessary to ensure the proper awarding of points according to the rules. This may mean reading another question, giving points to another quizzer, calling a prejump, etc." }
			h4 { "2.17.15 Penalties for lost appeal.  Some appeals bring point penalties if overruled. All points lost are subtracted from the team total, not the point total of the challenging quizzer (except in Individual Competition)." }

            //***************************************************************************
			h3 { "2.18 APPEAL OF QUESTION" }
			h4 { "2.18.1 Defined.  An appeal of the question is any objection legally raised concerning anything about a question except how it is \"introduced\" (which is an appeal of Procedure) as long as the question has been finished by the Quizmaster and not by a quizzer. (Questions finished by quizzers are prejumps - see Appeal of the Prejump.) In other words, an appeal of the question concerns the \"question type\" noted by the Quizmaster in advance, the way the question is read, the question itself, the way it was repeated, or the absence of any feasible answer to the question as read. The opposing captain will be given the allotted time for rebuttal." }
			h4 { "2.18.2 An appeal of the question should be raised and must be accepted when:" }
			h4 { "2.18.2.1 The question as written has no answer or no answer which can be feasibly stated within 20 seconds. The Quizmaster\'s determination is final." }
			h4 { "2.18.2.2 The question has been read by the Quizmaster in violation of the \"legal\" reading guidelines given." }
			h4 { "2.18.2.3 The question has more or fewer parts or is of a different \"question type\" than noted by the Quizmaster in the introduction (unless the question was completed by the Quizmaster prior to the first jump), or if it does not follow any of the other stated guidelines for \"legal questions.\"" }
			h4 { "2.18.2.4 The Quizmaster has added extra pertinent information in rereading a prejumped question that has been missed." }
			h4 { "2.18.3 Point penalty.  No points are placed in jeopardy with an appeal of the question if it is raised prior to any ruling on the first answer. However, after an announced ruling of an answer, an appeal of the question carries with it a 20-point penalty if it is overruled." }

            //***************************************************************************
			h3 { "2.19 APPEAL OF ANSWER" }
			h4 { "2.19.1 Defined.  An appeal of the answer is any objection legally raised concerning the correctness of a quizzer\'s answer, regardless of whether the question was a prejump completion or was finished by the Quizmaster, or whether it had been ruled \"correct\" by the Quizmaster." }
			h4 { "2.19.2 Accepted when ... An appeal of the answer must be accepted only when it can be demonstrated that according to the rules that the Quizmaster should have ruled differently on the quizzer\'s answer." }
			h4 { "2.19.3 Penalty.  Any appeal of an answer carries with it a 20-point penalty if overruled." }

            //***************************************************************************
			h3 { "2.20 APPEAL OF QUESTION THAT IS PREJUMPED" }
			h4 { "2.20.1 Defined.  An appeal of a prejump completion is any objection legally raised concerning the acceptability of a quizzer\'s completion to a prejump question." }
			h4 { "2.20.2 Context. This appeal is allowed when a quizzer\'s completion has been ruled incorrect and, after hearing the completed question, the quizzer (or captain) believes the rejected completion would have required the same information as the answer to the question as written.  This appeal is not allowed to reverse the Quizmaster\'s decision to allow a quizzer to proceed to answer a question based on his/her completion." }
			h4 { "2.20.3 Penalty.  An appeal of the prejump completion always carries a 20-point penalty if overruled." }

            //***************************************************************************
			h3 { "2.21 APPEAL OF PROCEDURE" }
			h4 { "2.21.1 Defined.  An appeal of a procedure is any objection legally raised concerning a rule violation which places the proper awarding of points in jeopardy, so long as the rule violation does not pertain to the question, answer, or prejump completion (which makes it an appeal of the question, answer, or prejump, respectively), but rather concerns some other rule or procedure which governs the fair awarding of points." }
			h4 { "2.21.2 Not allowed.  An appeal of procedure may not be raised if the fair awarding of points is not in question." }

            //***************************************************************************
			h3 { "2.22 WARNINGS AND INFRACTIONS" }
			h4 { "2.22.1 Illegal Conferring.  Quizzers cannot intentionally provide answers to team members or confer with each other, the coach, or with the audience by signal or voice from the time a question is begun until it is closed. A Quizmaster may give Warnings and Infractions to individual quizzers or to teams in violation.  No Scripture portions may be used by a quizzer in competition, though quizzers not in the competition at the moment (such as substitutes and those who quizzed out) may look in the Scriptures during the quiz." }
			h4 { "2.22.2 Illegal Discussion.  A question may not be discussed after it is closed if the Quizmaster is unwilling to discuss it. Quizzers, coaches, or fans who persist in this type of \"discussion\" may receive Warnings and Infractions. Coaches and quizzers do have the right to ask the quizmaster any question they wish between questions, and Quizmasters may answer if they wish. But if the Quizmaster indicates an unwillingness to discuss the point in question, the issue must be dropped.  (After the round is completed can be a good time for a quizzer or coach to ask a Quizmaster for insights on the ruling on a particular question.)" }
			h4 { "2.22.3 Warnings and Infractions.  Any Quizmaster or tournament director may give Warnings and/or Infractions to any team or individual who illegally confers, has illegal discussion, or exhibits disruptive or inappropriate behavior during any quiz or tournament. Warnings and Infractions will be given either to an individual or to a team, but not to both. Upon the third Warning, an Infraction will be given. The first Infraction will cause that team or individual to forfeit the question, with points automatically being awarded the other team. The second Infraction causes that team or individual to forfeit the quiz, and the third Infraction would mean elimination from the tournament. All Warnings and Infractions shall be recorded on the Official Scoresheet or Record Sheet with a capital \"W\" or \"I.\" While it is certain that these penalties will seldom if ever have to be used, it is hoped that their existence will foster closer observance of the rules and more discreet behavior. 

			Note: If a violation immediately jeopardizes the fair awarding of points, an Infraction may be given with no advance Warning." }

            //***************************************************************************
			h3 { "2.23 SPECIAL ACCOMMODATIONS" }
			h4 { "2.23.1 Jumping.  Quizzers may be granted an accommodation for purposes of a tournament if they are approved for this status by the tournament coordinator. If there is a disability such that normal jumping is either difficult or impossible, the quizzer may be allowed to compete with hand-held equipment, by activating the chair switch with the hand, or by any other mechanism so approved." }
			h4 { "2.23.2 Time Limit.  If a disability significantly slows a quizzer\'s rate of speech, they may qualify for a 30-second time limit in place of any 20-second time limit if approved for this by the tournament coordinator." }
			h4 { "2.23.3 Rulings.  If the quizzer\'s disability is such that speech is hard to understand or articulation difficult, Quizmasters may be more lenient in judgments of word clarity or allow more time for answering. The correct answer, however, must still be given in order for the quizzer to qualify for points.  Rereading consideration (with no loss of answer time) might be provided for those with special conditions like ADHD." }
			h4 { "2.23.4 Other Accommodations.  Any disability that affects a quizzer\'s ability to quiz should be taken into consideration on a case-by-case basis." }
			h4 { "2.23.5 Process.  Special needs and requests for accommodation should be brought to the tournament director for consideration ahead of the competition and, if granted, a notice shall be signed by the Tournament Director and presented to the Quizmaster in advance of the quiz.  The scorekeeper, timekeeper, captain and coach of the other team shall be informed about the nature of the accommodation to be granted." }

            //***************************************************************************
			h3 { "2.24 EQUIPMENT PROBLEMS" }
			h4 { "2.24.1 Defined.  An \"Equipment malfunction\" shall be called only when there is little or no doubt in the mind of the Quizmaster that the equipment is actually at fault. Any claim should be thoroughly investigated. A quizzer sitting precariously on the edge of the pressure point (where the least motion might activate the light) should be cautious in claiming an equipment failure. However, there may be legitimate cases of: 
			(a) a jump where the quizzer declares innocence, 
			(b) a light that seems to flicker or activate simply by moving the wires, or 
			(c) a seat pad demonstrating a reproducible delay in activating its light.

			The procedure in 2.17.4 should be followed for correcting the problem. The Quizmaster shall be the sole judge as to whether or not the equipment is to blame, and may take any necessary action to fairly continue with the question and deal with the problem." }
			h4 { "2.24.2 Discretion to address.  If the accuracy of the electronic equipment is questioned and its malfunctioning verified, the Quizmaster may alter normal proceedings in any way which is fair and impartial. This might include re-reading the question (in the case of a prejump where no key words were given), or reading a substitute question (if key words were given, or if a question was completed and it was impossible to tell which quizzer was really up first, etc.).
			" }
			h4 { "2.24.3 Questions replaced.  If the quizmaster determines that an equipment malfunction has possibly altered the outcome of a quiz, all affected questions shall be thrown out and replaced with new questions. If a prejump resulted because of the malfunction, the question shall be thrown out and a new question read for all the quizzers." }

            //***************************************************************************
            //***************************************************************************
            //***************************************************************************
			h2 { "3.   RESOURCES:  COACHES" }
			h3 { "3.1  COACHING" }
			h4 { "3.1.1 General.  The coach\'s responsibilities include encouraging spiritual growth in the lives of their quizzers (discipleship), prayerfully building and training a team, accompanying the team to tournaments, advising quizzers on strategy and game play (including the use of substitutions and time-outs), and representing the team to the local congregation and at large." }
			h4 { "3.1.2 Notes to the Coach: To build a team, relationships must be cultivated and enthusiasm generated. A good starter activity is to visit a quiz tournament with as many of your potential quizzers as possible. Enlisting the support of the Pastor, local Board of Administration, youth staff, and the parents of teens is crucial. Don\'t forget to let your Conference Bible Quiz Director know of your interest; he or she will be glad to give you valuable advice and support." }
			h4 { "3.1.3 Practices.  To train a team, schedule a regular time of practice; determine a study schedule (perhaps one chapter a week, with occasional reviews), employ principles of good teaching mixed in with asking questions over the text (teens need to understand in order to remember effectively); and supply loads of encouragement as needed. Take an interest in each individual quizzer, and communicate frequently about upcoming tournaments and retreats to keep excitement and study motivation high." }
			h4 { "3.1.4 Study tools.  Studying along with the quizzers also can serve as a valuable motivator and source of good fun and relationship building between you and your quizzers. You may wish to challenge your quizzers with some \"participation guidelines\" (like having read the chapter at least 3 times or knowing all chapter Quotes before each weekly practice), or some \"incentives\" (like taking the first quizzer who memorizes a certain chapter to McDonald\'s, etc.). 

			Note: Always be on the lookout for new quizzers, even after the quiz season is in progress." }
			h4 { "3.1.5 Tournaments.  Make sure to register properly in advance. Communicate with your church and parents of teens regarding all costs and scheduling for the trip, travel arrangements, etc.  It is suggested that a Parental Permission slip be collected from all parents." }
			h4 { "3.1.6 Register and set up.  When you arrive at a tournament, register and mobilize your quizzers, set up quiz seats and stand by for instructions. It is extremely important to make sure your chairs are working before you arrive at the quiz. Bring an extension cord to leave with the set." }

			h4 { "3.1.7 Pay attention during the quiz.  Be particularly alert during a quiz. This is not the time for the \"kids to do all the work.\" Pray with your team before each quiz. Appoint a team captain to represent your team\'s interests. Then follow along the best you can in your Bible. Give your prayer support and visible encouragement. Call a Time-Out if your team gets off to a slow start, and don\'t hesitate to use a substitute when you have a comfortable lead or when one or your \"regulars\" is having an \"off\" round. Be ready to appoint an \"acting captain\" if your captain is replaced by a substitute or quizzes out. Keep your own scoresheet and notes if possible to validate the Official Scoresheet and to discuss areas of possible improvement with your quizzers after the quiz. Be positive in your approach. Build their self-confidence and increase their desire to study God\'s Word." }
			h4 { "3.1.8 Sportsmanship is crucial. Cultivate a fine spirit among your team members by refraining from all criticism or judgment of the officials, other coaches, or quizzers. Make every effort by both your teachings and example to convey to quizzers the importance of applying God\'s Word that they have learned to life situations. The quiz is an ideal training ground. Quizzers keenly experience everything from the thrill of victory to the discouragement of defeat, and God\'s Spirit is needed to successfully handle both types of emotions. Help them to be sensitive to the needs of other people involved in the quiz: the officials, coaches, other quizzers, and even the fans. Let them know that God is always standing by to teach valuable lessons if we will let Him, and stress the fact that we often learn more from losing than winning. Better to lose points than tempers - quizzes than spiritual victory!" }
			h4 { "3.1.9 Advocate.  Be a spokesperson for the needs and interests of your quiz team in your local church. No other group has more to gain from a successful quiz ministry! Let them know that. Keep them informed of team activities and tournaments. Encourage them to pray, and don\'t be afraid to have an occasional fund-raiser to buy needed equipment or subsidize travel costs. Win or lose, maintain a spirit of optimism and confidence in your team. Remember, in Bible quizzing, everyone is a winner!" }
			h4 { "3.1.10 Engage.  You will also need to represent your team at tournaments and before your Conference and Regional quiz leaders. Share your ideas concerning new rules, rule interpretations, tournament schedules, and how they affect your ministry. Make positive suggestions for improvements." }

            //***************************************************************************
            //***************************************************************************
            //***************************************************************************
			h2 { "4.  RESOURCES:  QUIZMASTERS" }
			h3 { "4.1  QUIZMASTERING" }
			h4 { "4.1.1 Note to the quizmaster: As the key official in every quiz, you have the responsibility to see that the quiz is conducted smoothly, efficiently, and fairly. This means, first, that you will need to become familiar with all rules and procedures outlined in this Rulebook. Second, resist the natural tendency to be defensive in judgments and attitudes. Relax, be confident in your own abilities and in the knowledge that God will help you. And yet be open and honest when faced with an appeal or criticism. The rules provide for reversing decisions which are clearly in error. Be gracious in reversing incorrect decisions and upholding right ones. Your goal should always be two-fold: 
			1) to fairly award points according to the rules, honoring the team which has most effectively mastered the content of God\'s Word, and 
			2) to edify and strengthen in faith the hearts and minds of all those participating.
			" }
			h4 { "4.1.2 Demeanor.  Your attitude will communicate itself to the teams, officials, and audience. So be relaxed, make no hasty decisions, proceed efficiently, and SMILE!  Frequently quizzers are scared stiff. Your smile, with a little humor mixed in, will go a long way toward reducing the pressure and making a better quiz. Enjoy your ministry! You are working with teenagers of amazing potential and ability, and God will bless your efforts. In moments of tense decision-making, it will help to remember the ultimate aim and purpose of Bible quizzing.  The mission of Free Methodist Bible Quizzing is \"to instill biblical truths in the heart of teens, establish them in their faith, and prepare them for Christian service through the systematic study, meaningful application, and memorization of the word of God.\" Bible Quizzing is an avenue for hiding away the living Word of God in the minds and hearts of youth to equip them to live a life of love and service to Christ. Therefore, prayerfully seek and expect the help of the Holy Spirit in performing this task. Please execute faithfully the following responsibilities, and consult the sections referred to in parentheses when clarification is needed.
			" }
			h4 { "4.1.3 Preparation.  The Quizmaster\'s responsibilities for each quiz include personal preparation, appointing the necessary contest officials, mobilizing quizzers and testing jump equipment, and conducting a fair and impartial quiz within the time limit and conditions prescribed by the tournament coordinator. 
			" }
			h4 { "4.1.4 Learn the rules.  In preparing for the quiz, Quizmasters must first become familiar with all regulations and procedures contained in this Rulebook. Next, they must secure the questions and scoresheets from the tournament coordinator. Quizmasters should then read the questions silently before the quiz to familiarize themselves with the phraseology and punctuation.  The scoresheet should be marked with the date, name of tournament, round number, room number, and name of the officiating Quizmaster.  Later the team and quizzer information will be recorded during introductions.  Team information includes both the team number and name. 
			" }
			h4 { "4.1.5 Other officials.  The Quizmaster is to appoint a scorekeeper and a timekeeper and make sure they are familiar with all rules pertaining to their position. Appoint answer consultants if appropriate.  If comfortable doing so, or if no other persons are available to help, the Quizmaster may keep score and/or time, unless directed otherwise by the Tournament Director.  Trophy rounds should be planned in such a way that a scorekeeper is always available.
			" }
			h4 { "4.1.6 Checklist.  Before a quiz begins, Quizmasters should: 
			1) Have the captains choose sides on the basis of a coin toss or mutual agreement. 
			2) Have captains introduce themselves, their team members, and their coaches. As this occurs, the scorekeeper will write the names on the scoresheet and Quizmasters should write the quizzers\' names in the mini scoresheet on the question page to enable them to call on the quizzers by name for answering. 
			3) Introduce themselves and all assisting officials. 
			" }
			h4 { "4.1.7 Testing equipment.  After the quizmaster directs all quizzers to take their seats, the jump chairs should be tested to see that they are all working properly by giving quizzers some practice jumps on numbers, colors, claps, etc. The Quizmaster and timekeeper should now position themselves so that both can see the lights clearly. If some of the chairs are not working properly, one of three things should be done: 
			1) fix the problem, if the repair is as simple as securing a loose wire or resetting the box; 
			2) If it is a single pad malfunctioning, check the carrying case for a spare pad or consult the tournament coordinator to find another set of jump sets as a replacement; or 
			3) work out a fair and just solution with both coaches which will allow for the accurate recording of jumping order or which will at least spread the handicap evenly to both teams (like switching sides at Half-time, if one team perceives a disadvantage).
			" }
			h4 { "4.1.8 Pray and begin.  After beginning the contest with prayer, the Quizmaster shall conduct the quiz fairly and efficiently according to all rules contained in this Rulebook.
			" }
			h4 { "4.1.9 End of Quiz.  After the 15th question (or end of overtime - see C-9), the Quizmaster shall:
			(a) Declare the last question officially closed.
			(b) Congratulate the performance of all quizzers.
			(c) Announce the official score.
			(d) Have coaches check the scoresheet and make sure it is filled out completely.
			(e) Declare the quiz officially closed.
			(f) See that the official scoresheet is returned to the tournament coordinator or tournament statistician.
			(g) Prepare materials for next round of quizzing.
			" }
			h4 { "4.1.10 Outline for Quizmaster - A Brief Overview
			(a) Secure copy of the Rulebook and read thoroughly.
			(b) Get questions and official scoresheets from tournament coordinator.
			(c) Familiarize yourself with the reading of the questions.
			(d) Mark Scoresheet with pertinent data. 
			(e) Appoint Scorekeeper and Timekeeper (and others if needed).
			(f) Have captains pick sides, teams sit on quiz chairs, and the captains introduce their teammates.
			(g) Record names of quizzers on the Question sheet and have the scorekeeper write the names on the Scoresheet.
			(h) Introduce yourself and the other contest officials.
			(i) Review any rule changes and describe your style of quizmastering to quizzers.
			(j) Give quizzers practice jumps, and test the equipment.
			(k) Have prayer to start quiz.
			(l) Conduct the quiz, and remember Half-Time.
			(m) Declare the quiz over.
			(n) Deliver the scoresheet where directed.
			" }
			h4 { "4.1.11 Miscellaneous.  Also applicable during a quiz:
			(a) The Quizmaster should publicly announce all appeal deductions as soon as they apply (See Section J - Coach/Captain Appeals.)
			(b) The Quizmaster, even when calling on answer consultants for advice, makes the final decision on correctness. In difficult decisions, it is wise to seek insights from other quizmasters or even the tournament coordinator.
			(c) A Quizmaster shall not consider an answer until a Quizzer has been called upon to begin answering the question.
			(d) In the case of a prejump, a Quizmaster shall stop the reading of a question immediately. (See Prejumping.)
			(e) If no correct answer is given (or if no answers are given aloud in the case of a tie), it is a courtesy to give the correct answer to the quizzers and audience after the question is closed.
			(f) A question is closed after:
			   1) a Time-out has been granted by the Quizmaster, 
			   2) the Quizmaster starts the next question, or 
			   3) the Quizmaster announces \"The question is closed.\" After a question is closed, no appeals will be accepted and no rulings will be changed.
			" }
			h4 { "4.1.12 Procedures.  The Quizmaster is to observe proper procedures for reading questions, acknowledging jumps, ruling on Answers, and upholding all other rules during a quiz. If in error, the Quizmaster is to do whatever is necessary to see that points are awarded fairly." }
			h4 { "4.1.13 Quizmaster\'s Prerogative: A Quizmaster may change normal procedure (before the start of the next question) under either of these two conditions: 
			1) if it is necessary for the fair awarding of points in situations where following the rules would lead to an injustice; or 
			2) for personal preferences on procedures which are \"peripheral\" and not essential to the fair awarding of points.  
			In cases of personal preference, the Quizmaster must specify in advance the modification he or she is going to make so that all coaches and quizzers are aware of the difference in style. Prerogative of personal preference may obviously not be used to change \"basic rules\" on ruling, answering, etc. Only \"procedures\" may be adjusted. However, in adjusting normal procedure to make things fair (i.e. throwing out a question one quizzer has already heard, sending those participating in the appeal into the hall instead of those still eligible to answer when answers might be heard from another room, etc.), the Quizmaster may act without advance notice. Apart from these two conditions, however, a Quizmaster must go by the rules as closely as possible or be subject to an appeal of procedure.
			" }


            //***************************************************************************
            //***************************************************************************
            //***************************************************************************
			h2 { "5. FMBQ MINISTRY STRUCTURE" }
			h3 { "5.1 FMBQ LEADERSHIP ROLES" }
			h4 { "5.1.1 Tournament Coordinator.   The tournament coordinator may be any Conference or Regional Bible Quiz Director, Coach, or other quiz leader who sets up and runs a quiz tournament.  The coordinator\'s job is to plan, publicize, and administer the tournament according to the rules and guidelines contained in this Rulebook." }
			h4 { "5.1.1.1 Tournament Questions.  If running an \"official\" tournament, the tournament coordinator shall receive \"Official Questions\" from the Conference Director and shall print/copy in advance a sufficient quantity for all simultaneous quizzes to be run." }
			h4 { "5.1.1.2 Promote event.  The tournament coordinator must first decide who to invite to the tournament. Details of the tournament should be publicized by mail, email, phone, social media, other Internet-based platforms, or in person considerably in advance of the tournament. A formal registration form might be used to find out how many teams and individuals are coming, what categories of quizzing they will enter, how many sets of jump seats and quizmasters they can bring with them, the number of persons for meals, if they will need overnight lodging, etc." }
			h4 { "5.1.1.3 Quizmasters.  Find enough capable Quizmasters to run the tournament.  Invite members of host congregations to train as Quizmasters if needed." }
			h4 { "5.1.1.4 Quizmaster training.  Ask new Quizmasters to read copy of the Rulebook at least a week in advance of the tournament. Ask local quizzers to conduct a demo quiz to give the new Quizmasters a real-life understanding of the mechanics and pacing of a quiz.  Arrange a Quizmasters\' meeting to explain the schedule and details of the tournament." }
			h4 { "5.1.1.5 Materials. The Tournament Coordinator assembles all of the materials needed to run the tournaments: sets of questions in sufficient supply, scoresheets (if using digital scoresheets, the headings-team, round, room, QM-can be completed in advance and emailed to each Quizmaster), copies of Rulebooks for Quizmasters, schedules of the day\'s activities (including rounds, rooms, Quizmasters, and competing teams), maps of the host location and a Registration Form to help with team registration and statistics." }
			h4 { "5.1.1.6 Assistance.  Finally, other helpers may be needed to run the tournament. A statistician, spare Quizmaster, fix-it person for jump seats, cooks, even a babysitter, are possibilities. The coordinator for larger tournaments should avoid being scheduled as a Quizmaster to ensure availability to handle questions and logistical needs." }
			h4 { "5.1.1.7 Coordinator\'s Prerogative. Because Bible quizzing involves innovation and experimentation at the level of conference ministry and local tournaments, tournament coordinators have the privilege of revising rules and procedures and trying new ones at their discretion. The only requirements are that all changes be cautiously chosen and clearly communicated in advance to all coaches and quizzers participating at the tournament." }
			h4 { "5.1.2 Conference Bible Quiz Director.  The Conference Bible Quiz Director (identified, approved and/or appointed by the Regional Director or Denominational Director) is to pray for and encourage the coaches of the conference, to promote FMBQ in the Conference, to arrange for tournaments on a regular basis, and to represent the ministry needs of the area to the Regional and Denominational quiz directors." }
			h4 { "5.1.2.1 Promotion ideas.  Mail or email a letter of invitation to all conference pastors; report tournament results in conference publications; target a few churches for \"sponsoring\" by existing teams who want to involve themselves in the ministry of planting new teams; and perhaps most importantly, keep those currently in quizzing involved in regular, meaningful competition and fellowship. Send a newsletter on a regular basis to all conference coaches; make telephone calls to encourage coaches." }
			h4 { "5.1.2.2 Tournament Questions.  It is the responsibility of the Conference Bible Quiz Director to secure Official Tournament Questions from the Denominational Director and to keep them confidential at all times. These questions should be given only to a designated Tournament Coordinator who has the job of overseeing a tournament, the Quizmasters involved in that tournament, and answer consultants (if they are used)." }
			h4 { "5.1.2.3 Schedules.  In addition to securing tournament questions, an annual schedule of conference competition should be compiled, seeking guidance from the Regional Director and considering the suggestions and input of area coaches. (It is wise to consult with neighboring conference directors to schedule tournaments to better coordinate planning, and to alternate \"conference\" with \"regional\" tournaments.) Tournament Coordinators should be appointed for each tournament. The tournament schedule should be publicized as soon as it is confirmed so that coaches can plan accordingly." }

			h4 { "5.1.2.4 Communication.  The Conference Director is to represent quizzing in the assigned area by regular communication with 
			1) the area\'s Regional Bible Quiz Coordinator, 
			2) local pastors in the Annual Conference, 
			3) the Denominational Director, and 
			4) any others who need updates in various areas so that they can provide effective help and support." }

			h4 { "5.1.3  Regional Bible Quiz Director.  Regional Bible Quiz Directors are appointed by the Denominational Director to promote and oversee Bible Quizzing in their regions. They shall pray for and communicate frequently with the Conference Bible Quiz Directors in their region, and report tournament news and ministry needs to the Denominational Director and/or anyone on staff who is responsible for publicizing this news or providing support for the ministry. They shall also serve on the FM Bible Quiz Leadership Team (which shall recommend policy and rule modifications for Bible Quizzing) along with the Denominational Director. A more detailed ministry description is available from the Denominational Director." }
			h4 { "5.1.4 Denominational Bible Quiz Director.  The Denominational Director is responsible for promoting Bible Quizzing at the denominational level, providing prayer support and encouragement to quizzing leadership, providing materials for church and conference quizzing ministries, and responding to other local needs as they arise. The director is responsible for coordinating the annual denominational Bible Quiz Finals, and for chairing the Bible Quiz Leadership Team, holding the final authority on all policy and rule decisions." }

        },
    )
}