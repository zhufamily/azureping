<h1>A (very) simple Rust based AI project</h1>
<h2>Setup</h2>
The project is set up as following.
<ul>
  <li>Windows 11 OS</li>
  <li>Rustrover IDE</li>
  <li>Docker Desktop</li>
  <li>An existing onnx file</li>
</ul>
To run the project, follow the steps below.
<ol>
  <li>Make sure Docker Desktop is running</li>
  <li>Open the project in Rustrover</li>
  <li>In terminal window inside Rustrover, type "docker compose up -d --build"</li>
  <li>Verify the image is generated and running in Docker</li>
  <li>Open browser and make sure http://localhost:1024 is responding</li>
  <li>To predict a single item, go to test folder, run command "curl -X POST http://localhost:1024/predict -H "Content-Type: application/json" --data @data.json"</li>
  <li>To conduct a batch prediction, go to test folder, run command "Go to test folder, run command "curl -X POST http://localhost:1024/predict -H "Content-Type: application/json" --data @data.json"</li>
</ol>
<h2>Take Aways</h2>
After playing Rust over the x-mas to new year vacation time, here are some high-level take aways (welcome to comment).
<h3>No system will be 100% memory safe</h3>
Lots of claims on the internet that switch to Rust will give you a memory safe system, based on my research a safe system never comes with switch to a new language!
<br/>
First of all, no system can run without hardware, any interaction to hardware or memory management, cannot be memory safe period.  There will have to a C++ or Rust unsafe layer for that.
<br/>
Second, Rust itself due to the design concept of ownership and borrow, it will have difficulties handling bidirectional relationship.  Given the simplest data structure for double linked list, inside Rust it is actually implemented with unsafe layer, though it should be memory safe through a battle grade testing.  However, that is not memory safe by the language but through good design patterns and rigouous testing, no differences from any other languages. 
<br/>
Third, practically speaking, Rust cannot compete with decades of libraries for C++/Java/C#/Python, in order to get "financially" sound, tons of wrappers/bridges are adopted everywhere.  In my sample, the load / execution of onnx model file is a C++ wrapper, which apparently leads to some unsafe layer of codes.  Would it be possible to rewrite everything with safe layer?  The consensus from the community is clearly a "no". 
<h3>Why you should (or should not) go Rust</h3>
If dealing with hardware/memory management directly, no safe layer will be possible with the currect technology.  Therefore, most likely switch to Rust will not buy you memory safe, but if there are other reasons for switch then it maybe benifical.
<br/>
If dealing with business logic, the benefits of getting rid of GC style of memory management for Java / C# is very marginal.  The focus is to get business logic correct and fit into complicated context for daily operations.
<br/>
If dealing with realtime system with no direct hardware access, that should be the sweet spot, so most efforts now are inside cloud vendors, database engines etc.  In those systems, performance and relibility is critical, and they do not always touch hardware directly.
<h3>Should you adopt a hybrid approach</h3>
Given the ecosystem size for other languages, most proposed approach is hybrid / combined Rust with existing stuff.  However, some serious considerations should be taken in the first place before the jump.
<br/>
Rust is designed from a very different paradigm -- it does not have class/inheritance, it enforces memory ownership, it handles parallel differently -- all of those pose some (or a lot) design conflicts among the team.  That can be very clearly see from the debate of introduction of Rust into Linux.
<br/>
A more practical question is where to draw the line -- only use Rust for new things / partially rewrite old system / complete overhaul -- the decision will take a very long time to come if truely based on business / technical analysis; most nowadays, we are seeing marketing slogans rather than real analytical results.
<br/>
As a common sense, start with baby steps, and then if there are real/tangible benefits, pour in more resources.  One thing to point out, even for experienced developers, it will be considerable amount of time before they can be proficient in Rust, due to the different design approach.
<h3>What about the project</h3>
As a user not a cloud service provider, the benefits are very marginal if any, switch to Rust from Python.
<br/>
In Python, it might be only several lines of codes, and within an hour, it will spin up a endpoint for inference from a trained model.  On the other hand, with help of the latest AI tool, it took me 20 hours to reach to the stage, which still lacks capabiliteis for scaling, error logging and handling etc.  Most challenges are from the crate version hell and c++ wrapper.  The very same thing as dll hell, there is something called crate version hell -- some crates pointing to the same dependencies but different versions.  For c++ wrapper -- ort crate, it needs a fully loaded Linux image with c++ compiler and other libraries such as openssl (cannot be a minimized Linux, I chooced Ubuntu instead), which makes the building process cumbersome, and the final image very big.  
<br/>
Though, it does get rid of cold start -- in terms of 10 to 20 seconds for the first request, or memory buffer crash issue -- not happening for 99%+ of cases.
<br/>
In summary, it is a good learn process for conduct some AI work (or micro-service framework) with Rust, but it can hardly make any commercial sense.
