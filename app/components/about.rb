class About
  include Inesita::Component

  def render
    h3 "What is this?"
    p do
      p class: "image" do
        img src: "/static/images/fazic.jpg"
      end
      a href: "/fazic" do
        'FAZIC '
      end
      span "is a fantasy retro computer. You can create, share, and play tiny games or programs. You have build in BASIC interpreter, so you can start now like it's the 80s."
    end
      a href: "/fazic" do
        'CLICK HERE TO START'
      end
    h3 "What now?"
    p do
      span "When you will see READY prompt it's time to write your first basic program. Don't know basic? Go to"
      a href: router.url_for(:help) do
        ' HELP '
      end
      span "section. You'll see how to write your first basic program, available commands, and everything you need to start."
    end
    a href: router.url_for(:help) do
      'I NEED HELP'
    end
    h3 "Can I save or load my programs?"
    p do
      span "Yes. Any time you want. Just type SAVE \"NAME\" and thats it. You can go to"
      a href: router.url_for(:programs) do
        ' PROGRAMS '
      end
      span "section to see you programs or copy them. Of course you can load them by typing LOAD \"NAME\"."
    end
    a href: router.url_for(:programs) do
      'SHOW MY SAVED PROGRAMS'
    end
    h3 "Is it everything?"
    p do
      span "No. If you want to see what is planned go to"
      a href: router.url_for(:roadmap) do
        ' ROADMAP '
      end
      span "section. You'll find there all things that will happen in the future. "
    end
    a href: router.url_for(:roadmap) do
      'SHOW ME THE ROADMAP'
    end
    h3 "How can I thank you?"
    p do
      span "If you like it, check"
      a href: router.url_for(:roadmap) do
        ' PATREONS '
      end
      span "section now."
    end
    a href: router.url_for(:roadmap) do
      'SHOW ME THE ROADMAP'
    end
  end
end
