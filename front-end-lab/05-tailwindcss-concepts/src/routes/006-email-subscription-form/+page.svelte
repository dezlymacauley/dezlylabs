<!-- flex justify-center items-center -->
<div class="bg-gray-950 min-h-screen">
  <div class="bg-purple-700">
    <img src="/white_haired_woman.png" alt="a white haired woman" class="w-80 rounded-4xl p-4">
  </div>
  <h1 class="text-white">Snowfall Models</h1>
  <p class="text-white">
  Sign up with Snowfall Models to showcase your talent and connect with top industry opportunities. Let your modeling journey begin here.
  </p>

  <section>
    <form 
      action="http://127.0.0.1:5261/signup" 
      method="post" 
      target="_blank"
    >
   
    <!-- 
      `fieldset` is a semantic tag that let's you group related 
      sections is an html form
    -->
    <fieldset>
      <!-- 
        This is a semantic tag that provides a description 
        for the fieldset 
      -->
      <legend>New user details</legend>

      <!-- `label` is a semantic tag for each input within a fieldset -->
      <label>
        <input type="text" name="reviewer_name" placeholder="Enter your name" required>
      </label>

      <br>
      <br>

      <label>
        <input type="email" name="reviewer_email" placeholder="Enter a valid email">
      </label>

    </fieldset>

    <!-- I like to have a fieldset just for the submit button -->
    <fieldset>
      <legend>Registration Button</legend>
      <button type="submit" class="bg-green-500">Register Now</button>
    </fieldset>

    </form>
  </section>

</div>
